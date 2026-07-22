use std::sync::Arc;

use axum::serve;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::{banner::display_banner, state::AppState};
use crate::{errors::ProviderError, routes::app};

mod auth;
mod banner;
mod config;
mod errors;
mod handlers;
mod routes;
mod sources;
mod state;
mod types;

#[tokio::main]
async fn main() -> Result<(), ProviderError> {
    // Initialize tracing consumer
    // Set the environment variable `RUST_LOG=<level>` to display logging in console
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let state = AppState::from_config()?;
    let listener = tokio::net::TcpListener::bind(format!(
        "{}:{}",
        state.config.address, state.config.port
    ))
    .await?;
    info!(?state.config.address, state.config.port, "Starting TMS Provider app");
    let banner = display_banner(&state);
    debug!(banner);
    if !state.config.silent {
        println!("{banner}");
    }
    serve(listener, app(Arc::new(state))).await?;
    Ok(())
}
