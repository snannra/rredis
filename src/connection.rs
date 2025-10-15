use crate::error::Error;
use crate::response::Response;
use anyhow::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::task::spawn;
use tracing::info;

enum Command {
    Ping,
    Echo(String),
    Unknown,
}

pub async fn handle_connection(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
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
                let msg = String::from_utf8_lossy(&buf[..]).trim().to_string();

                let response = response(&msg);

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

fn response(msg: &str) -> Response {
    let args = msg.split_whitespace().collect::<Vec<&str>>();

    let cmd = match args.get(0) {
        Some(&"PING") => Command::Ping,
        Some(&"ECHO") => {
            let echo_msg = args[1..].join(" ");
            Command::Echo(echo_msg)
        }
        _ => {
            info!("Unknown command: {}", msg);
            Command::Unknown
        }
    };

    match cmd {
        Command::Ping => Response::Simple("PONG".into()),
        Command::Echo(msg) => Response::Bulk(Some(msg)),
        Command::Unknown => Error::UnknownCommand(msg.to_string()).to_response(),
    }
}
