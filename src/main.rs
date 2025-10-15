use clap::Parser;
use rredis::{config::Config, server::Server, shutdown, telemetry};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    telemetry::init();

    let config = Config::parse();
    let server = Server::new(config.get_addr(), config.get_max_conns());

    let shutdown_signal = shutdown::ctrl_c();
    tokio::pin!(shutdown_signal);

    server.run(&mut shutdown_signal).await?;

    Ok(())
}
