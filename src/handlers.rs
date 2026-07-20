use std::sync::Arc;

use crate::auth::UserClaims;
use axum::extract::State;
use axum_responses::JsonResponse;

use crate::{errors::ServiceError, state::AppState};

pub async fn resources(
    State(state): State<Arc<AppState>>,
    UserClaims(claims): UserClaims,
) -> Result<JsonResponse, ServiceError> {
    let resources = state.source.get_resources(claims.subject).await?;
    Ok(JsonResponse::Ok()
        .message("success")
        .data(resources))
}
