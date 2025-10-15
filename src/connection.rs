use anyhow::Result;
use tracing::info;

pub async fn handle_connection(
    _socket: tokio::net::TcpStream,
    addr: std::net::SocketAddr,
) -> Result<(), anyhow::Error> {
    info!("Handling connection from {}", addr);

    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    info!("Handled connection from {}, closing connection.", addr);

    Ok(())
}
