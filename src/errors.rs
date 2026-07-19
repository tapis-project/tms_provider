use axum::http::StatusCode;
use axum_responses::{HttpError, JsonResponse};
use thiserror::Error;

use crate::types::{AccountId, ProviderId, ResourceId};

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("TMS Provider was unable to read application config file; details: {0}")]
    ApplicationConfigFileError(#[from] std::io::Error),
    #[error("TMS Provider was unable to read configuration sources; details: {0}")]
    ApplicationConfigError(String),
}

#[derive(Error, Debug, HttpError)]
pub enum ServiceError {
    #[error("Data source error: {0}")]
    #[http(transparent)]
    FromSource(#[from] SourceError),
    #[error("Authentication error: {error}")]
    #[http(code = 403, error = error)]
    AuthenticationError { error: String },
}

#[derive(Error, Debug)]
pub enum SourceError {
    #[error("Provider not found: {0}")]
    ProviderNotFound(ProviderId),
    #[error("Account not found: {0}")]
    AccountNotFound(AccountId),
    #[error("Resource not found: {0}")]
    ResourceNotFound(ResourceId),
    #[error("I/O Error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Generic error: {0}")]
    GenericError(StatusCode, String),
}

impl From<SourceError> for JsonResponse {
    fn from(value: SourceError) -> Self {
        match value {
            err @ (SourceError::ProviderNotFound(_)
            | SourceError::AccountNotFound(_)
            | SourceError::ResourceNotFound(_)) => {
                JsonResponse::NotFound().error(err.to_string())
            }
            SourceError::IOError(error) => {
                JsonResponse::InternalServerError().error(error.to_string())
            }
            SourceError::GenericError(status_code, error) => {
                JsonResponse::status(status_code).message(error)
            }
        }
    }
}
