/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

#[tokio::main]
pub async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    // Trace an event outside of any span context.
    tracing::event!(tracing::Level::INFO, "hello world");
}
