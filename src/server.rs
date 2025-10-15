use crate::connection::handle_connection;
use anyhow::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Semaphore;
use tracing::{error, info, warn};

pub struct Server {
    addr: String,
    max_conns: usize,
}

impl Server {
    pub fn new(addr: String, max_conns: usize) -> Self {
        Self { addr, max_conns }
    }

    pub async fn run(
        &self,
        mut shutdown: impl std::future::Future<Output = ()> + Unpin,
    ) -> Result<()> {
        let listener = TcpListener::bind(self.addr.clone()).await?;
        info!("Listening on {}", self.addr);

        let limiter = Arc::new(Semaphore::new(self.max_conns));

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
}
