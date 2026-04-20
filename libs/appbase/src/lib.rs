use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the common application base: structured logging with env filter.
/// Call this once at the start of every Rust service.
pub fn init() {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .json()
        .init();

    logging::register_default_metrics();
    tracing::info!("appbase initialized");
}
