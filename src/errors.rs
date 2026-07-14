use axum::response::IntoResponse;
use axum_responses::JsonResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("TMS Provider was unable to read application config file; details: {0}")]
    //#[http(code = 500)]
    ApplicationConfigFileError(#[from] std::io::Error),

    #[error("TMS Provider was unable to read configuration sources; details: {0}")]
    //#[http(code = 500, message = "App config error")]
    ApplicationConfigError(String),
}

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("Provider Error: {0}")]
    //#[http(transparent)]
    FromProvider(#[from] ProviderError),
}

impl From<ServiceError> for JsonResponse {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::FromProvider(provider_error) => {
                JsonResponse::InternalServerError().error(format!("{provider_error}"))
            }
        }
    }
}

impl IntoResponse for ServiceError {
    fn into_response(self) -> axum::response::Response {
        JsonResponse::from(self).into_response()
    }
}
