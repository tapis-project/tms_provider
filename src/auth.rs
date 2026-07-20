use std::sync::Arc;

use axum::{
    RequestPartsExt,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jwtiny::{AlgorithmPolicy, Claims, ClaimsValidation, TokenValidator};
use moka::future::Cache;
use reqwest::Client;

use crate::{config::ApplicationConfig, errors::ServiceError, state::AppState};

const CACHE_MAX_CAPACITY: u64 = 1000;

pub fn mk_validator(config: &ApplicationConfig) -> TokenValidator {
    let client = Client::new();
    let cache = Cache::<String, Vec<u8>>::builder()
        .time_to_live(config.jwt_key_cache_ttl)
        .max_capacity(CACHE_MAX_CAPACITY)
        .build();

    let issuers = config
        .jwt_issuers
        .as_ref()
        .unwrap_or(&Default::default())
        .iter()
        .map(|url| url.to_string())
        .collect::<Vec<_>>();
    TokenValidator::new()
        .algorithms(AlgorithmPolicy::rsa_all())
        .issuer(move |iss| issuers.contains(&iss.to_string()))
        .validate(
            ClaimsValidation::default()
                .no_nbf_validation()
                .no_iat_validation(),
        )
        .jwks(client)
        .cache(cache)
}

//pub struct UserClaims(Claims);
#[derive(Debug)]
pub struct UserClaims(pub Claims);

impl<S> FromRequestParts<S> for UserClaims
where
    Arc<AppState>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ServiceError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = Arc::<AppState>::from_ref(state);
        let maybe_bearer = parts.extract::<TypedHeader<Authorization<Bearer>>>().await;
        let token = maybe_bearer
            .map_err(|err| ServiceError::AuthenticationError {
                error: format!("{err}"),
            })?
            .token()
            .to_owned();
        dbg!(&token);
        Ok(UserClaims(state.validator.verify(&token).await.map_err(
            |err| ServiceError::AuthenticationError {
                error: format!("JWT token error: {err}"),
            },
        )?))
    }
}
