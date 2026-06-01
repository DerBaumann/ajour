use tracing_subscriber::{self, layer::SubscriberExt, util::SubscriberInitExt};

pub fn setup_logging() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().pretty())
        .init();
    std::panic::set_hook(Box::new(|panic_info| {
        tracing::error!("panic: {:?}", panic_info)
    }));
}
