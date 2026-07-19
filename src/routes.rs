use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{handlers::resources, state::AppState};

pub fn app(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/resources", get(resources))
        .with_state(state)
}
