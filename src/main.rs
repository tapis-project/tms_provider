use std::sync::Arc;

use axum::serve;
use indoc::formatdoc;
use tracing::{debug, info};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::state::AppState;
use crate::{errors::ProviderError, routes::app};

mod auth;
mod config;
mod errors;
mod handlers;
mod routes;
mod sources;
mod state;
mod types;

fn banner(state: &AppState) -> String {
    let version = &state.version;
    let rust_version = &state.rust_version;
    let commit = &state.commit;
    let data_source = &state.config.source_kind;
    let address = &state.config.address;
    let port = &state.config.port;
    formatdoc!(
        r#"
        --- TMS Resources Provider ---
        Version: {version}
        Commit: {commit}
        Rust version: {rust_version}

        Using Data source: {data_source:?}
        Listening at: {address}:{port}
    "#
    )
}

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
    let banner = banner(&state);
    debug!(banner);
    if !state.config.silent {
        println!("{banner}");
    }
    serve(listener, app(Arc::new(state))).await?;
    Ok(())
}
