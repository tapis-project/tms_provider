use std::{path::PathBuf, sync::Arc};

use crate::{errors::SourceError, sources::Source};
use axum::{
    extract::{Path, State},
    http::HeaderMap,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
    typed_header::TypedHeaderRejection,
};
use axum_responses::JsonResponse;
use uuid::Uuid;

use crate::{
    errors::{ProviderError, ServiceError},
    sources::{Resource, Resources},
    state::AppState,
};

pub async fn resources(
    State(state): State<Arc<AppState>>,
    maybe_bearer: Result<TypedHeader<Authorization<Bearer>>, TypedHeaderRejection>,
) -> Result<JsonResponse, ServiceError> {
    // validate token
    // get account from token
    let token = maybe_bearer
        .map_err(|err| ServiceError::AuthenticationError {
            error: format!("{err}"),
        })?
        .token()
        .to_owned();
    dbg!(token);
    let resources = state.source.get_resources(None).await?;
    Ok(JsonResponse::Ok()
        .message("success from resources handler")
        .data(resources))
}
