use std::sync::Arc;

use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::errors::ProviderError;
use crate::state::AppState;

mod types;
mod config;
mod errors;
mod routes;
mod state;
mod handlers;
mod sources;
mod auth;

#[tokio::main]
async fn main() -> Result<(), ProviderError> {
    // Initialize tracing consumer
    // Set the environment variable `RUST_LOG=<level>` to display logging in console
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState::from_config()?;

    let address = state.config.address;
    let port = state.config.port;
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}",)).await?;

    let app = routes::app(Arc::new(state));
    info!(?address, port, "Started TMS Provider");
    axum::serve(listener, app).await?;
    Ok(())
}
