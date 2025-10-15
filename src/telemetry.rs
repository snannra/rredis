use tracing_subscriber::{self, EnvFilter, util::SubscriberInitExt};

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .finish()
        .init();
}
