use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(name = "rredis", about = "A simple Redis clone in Rust")]
pub struct Config {
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
    pub fn get_addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    pub fn get_max_conns(&self) -> usize {
        self.max_conns
    }
}
