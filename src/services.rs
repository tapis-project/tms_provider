use std::{path::PathBuf, sync::Arc};

use crate::sources::file::FileSource;
use axum::extract::{Path, State};
use axum_responses::JsonResponse;
use uuid::Uuid;

use crate::{
    errors::{ProviderError, ServiceError},
    provider::get_resources,
    sources::{Resource, Resources},
    state::AppState,
};

pub async fn resources(
    State(state): State<Arc<AppState>>,
    Path((provider_id, provider_account_id)): Path<(String, String)>,
) -> Result<JsonResponse, ServiceError> {
    let s = FileSource {
        file: PathBuf::from("foo"),
    };
    let resources = get_resources(&s, provider_id, provider_account_id)?;
    Ok(JsonResponse::Ok().message("success").data(resources))
}
