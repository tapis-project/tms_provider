use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tracing::debug;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::{config::ApplicationConfig, errors::ProviderError};

mod config;
mod errors;

#[tokio::main]
async fn main() -> Result<(), ProviderError> {
    // Initialize tracing consumer
    // Set the environment variable `RUST_LOG=<level>` to display loggin in console
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let app_config = ApplicationConfig::from_sources()?;

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    debug!(route = "foo", "App started");

    let address = app_config.address;
    let port = app_config.port;
    let listener = tokio::net::TcpListener::bind(format!("{address}:{port}",)).await?;
    debug!(?address, port, "Started TMS Provider");
    axum::serve(listener, app).await?;
    Ok(())
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
