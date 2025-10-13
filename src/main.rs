use clap::Parser;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tracing::*;
use tracing_subscriber::{self, util::SubscriberInitExt};

#[derive(Parser, Debug, Clone)]
#[command(name = "rredis", about = "A simple Redis clone in Rust")]
struct Config {
    /// Interface to bind
    #[arg(long, default_value = "127.0.0.1")]
    bind: String,

    /// TCP port to listen on
    #[arg(long, default_value_t = 6380)]
    port: u16,

    /// Maximum number of concurrent client connections
    #[arg(long, default_value_t = 1024)]
    max_conns: usize,
}

impl Config {
    fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish()
        .init();

    info!("Starting rredis...");

    let config = Config::parse();
    info!("Parsed config");

    let addr = config.addr();

    let listener = TcpListener::bind(addr.clone()).await?;
    info!("Listening on {}", addr);

    let limiter = Arc::new(Semaphore::new(config.max_conns));

    let shutdown = async {
        tokio::signal::ctrl_c().await.expect("signal");
    };

    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            res = listener.accept() => {
                match res {
                    Ok((socket, peer_addr)) => {
                        let permit = limiter.clone().acquire_owned().await.unwrap();
                        tokio::spawn(async move {
                            let _permit = permit;
                            if let Err(e) = handle_connection(socket, peer_addr).await {
                                warn!("Error handling connection from {}: {}", peer_addr, e);
                            }
                        });
                    }
                    Err(e) => {
                        error!("Failed to accept connection: {}", e);
                        continue;
                    }
                }
            }
            _ = &mut shutdown => {
                info!("Shutdown signal received, shutting down...");
                break;
            }
        }
    }

    Ok(())
}

async fn handle_connection(
    socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), anyhow::Error> {
    info!("Handling connection from {}", addr);

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    info!("Handled connection from {}, closing connection.", addr);

    Ok(())
}
