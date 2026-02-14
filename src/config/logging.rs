use tracing_subscriber::EnvFilter;

pub fn build_env_filter() -> EnvFilter {
    EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info,torvi=debug"))
}

pub fn init() {
    tracing_subscriber::fmt()
        .with_env_filter(build_env_filter())
        .with_target(true)
        .init();
}
