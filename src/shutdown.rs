use tracing::info;

/// Returns a future that resolves on Ctrl+C
pub async fn ctrl_c() {
    // Propagate errors as panic because shutdown should always work
    tokio::signal::ctrl_c()
        .await
        .expect("failed to install Ctrl+C handler");
    info!("Shutdown signal received");
}
