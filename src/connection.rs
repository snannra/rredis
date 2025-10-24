use crate::error::Error;
use crate::response::Response;
use crate::store::{Database, Entry, KvStore};
use anyhow::Result;
use std::time::Instant;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::spawn;
use tracing::info;

enum Command {
    Ping,
    Echo(String),
    Get(String),
    Set(String, String),
    Del(Vec<String>),
    Exists(Vec<String>),
    ExpireAt(String, Instant),
    Persist(String),
    TtlMs(String),
    Unknown,
}

pub async fn handle_connection(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
    db: Database,
) -> Result<(), Error> {
    info!("Handling connection from {}", addr);

    let (mut reader, mut writer) = socket.into_split();

    info!("Spawned reader and writer.");

    let reading = spawn(async move {
        loop {
            let mut buf: Vec<u8> = vec![0u8; 1024];
            let n = match reader.read(&mut buf).await {
                Ok(n) => n,
                Err(e) => {
                    info!("Error reading from {}: {}", addr, e);
                    return Error::InvalidUtf8.to_response();
                }
            };
            if n != 0 {
                info!("Read {} bytes from {}", n, addr);
                let msg = String::from_utf8_lossy(&buf[..n]).trim().to_string();

                let response = response(&msg, db.clone()).await;

                writer
                    .write(&response.to_bytes())
                    .await
                    .unwrap_or_else(|e| {
                        info!("Error writing to {}: {}", addr, e);
                        0
                    });
            } else {
                info!("Connection closed by {}", addr);
                return Error::Empty.to_response();
            }
        }
    });

    let _ = reading.await;

    info!("Handled connection from {}, closing connection.", addr);

    Ok(())
}

async fn response(msg: &str, db: Database) -> Response {
    let args = msg.split_whitespace().collect::<Vec<&str>>();

    let cmd = match args.get(0) {
        Some(&"PING") => Command::Ping,
        Some(&"ECHO") => {
            let echo_msg = args[1..].join(" ");
            Command::Echo(echo_msg)
        }
        Some(&"GET") => {
            if args.len() == 2 {
                Command::Get(args[1].to_string())
            } else {
                Command::Unknown
            }
        }
        Some(&"SET") => {
            let mut index_of_opts = args.len();
            for i in 1..args.len() {
                if args[i].to_uppercase() == "NX"
                    || args[i].to_uppercase() == "XX"
                    || args[i].to_uppercase() == "DEFAULT"
                {
                    index_of_opts = i;
                    break;
                }
            }
            if args.len() >= 3 {
                Command::Set(args[1].to_string(), args[2..].join(" ").to_string())
            } else {
                Command::Unknown
            }
        }
        Some(&"DEL") => {
            if args.len() >= 2 {
                let keys = args[1..].iter().map(|s| s.to_string()).collect();
                Command::Del(keys)
            } else {
                Command::Unknown
            }
        }
        Some(&"EXISTS") => {
            if args.len() >= 2 {
                let keys = args[1..].iter().map(|s| s.to_string()).collect();
                Command::Exists(keys)
            } else {
                Command::Unknown
            }
        }
        Some(&"EXPIREAT") => {
            if args.len() == 3 {
                if let Ok(timestamp) = args[2].parse::<u64>() {
                    let when = Instant::now() + std::time::Duration::from_secs(timestamp);
                    Command::ExpireAt(args[1].to_string(), when)
                } else {
                    Command::Unknown
                }
            } else {
                Command::Unknown
            }
        }
        Some(&"PERSIST") => {
            if args.len() == 2 {
                Command::Persist(args[1].to_string())
            } else {
                Command::Unknown
            }
        }
        Some(&"TTLMS") => {
            if args.len() == 2 {
                Command::TtlMs(args[1].to_string())
            } else {
                Command::Unknown
            }
        }
        _ => {
            info!("Unknown command: {}", msg);
            Command::Unknown
        }
    };

    match cmd {
        Command::Ping => Response::Simple("PONG".into()),
        Command::Echo(msg) => Response::Bulk(Some(msg)),
        Command::Get(key) => {
            let response = db.get(&key).await;
            match response {
                Some(value) => Response::Bulk(Some(String::from_utf8_lossy(&value).to_string())),
                None => Response::Bulk(None),
            }
        }
        Command::Set(key, value) => {
            let response = db
                .set(&key, value.into_bytes().into(), Default::default())
                .await;
            let mut data = db.data.write().await;
            let entry = Entry::new(value.into_bytes());
            let result = match data.insert(key, entry) {
                Some(_) => "Updated",
                None => "OK",
            };
            Response::Simple(format!("{}", result))
        }
        Command::Unknown => Error::UnknownCommand(msg.to_string()).to_response(),
    }
}

// async fn get(&self, key: &str) -> Option<Value>;
//     async fn set(&self, key: &str, val: Value, opts: SetOptions) -> bool;
//     async fn del(&self, keys: &[String]) -> usize;
//     async fn exists(&self, keys: &[String]) -> usize;
//     async fn expire_at(&self, key: &str, when: Instant) -> bool;
//     async fn persist(&self, key: &str) -> bool;
//     async fn ttl_ms(&self, key: &str) -> Option<i64>;
