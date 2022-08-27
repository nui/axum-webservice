use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

pub fn init() {
    let env_filter = EnvFilter::new("info,axum_webservice=debug");
    let stdout = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(stdout)
        .with(env_filter)
        .init();
}
