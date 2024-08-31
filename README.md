# Demo Rust Axum + tracing subscriber

Edit file `Cargo.toml`.

Add dependencies:

```toml
axum = { version = "0.7.5" } # Web framework that focuses on ergonomics and modularity.
tokio = { version = "1.40.0", features = ["full"] } # Event-driven, non-blocking I/O platform.
tracing = { version = "0.1.40" } #  Application-level tracing for Rust.
tracing-subscriber = { version = "0.3.18", features = ["env-filter"] } # Utilities for implementing and composing ```
```

Edit file `main.rs`.

Add code to use tracing:

```rust
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
    tracing::event!(tracing::Level::INFO, "main");
}
```

### Try the demo…

Shell:

```sh
cargo run
```

You should see console output that shows tracing such as:

```sh
2024-08-31T18:19:55.892163Z INFO demo_rust_axum_tracing_subscriber: main
```
