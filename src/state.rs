use std::{path::PathBuf, time::Duration};

use jwtiny::{AlgorithmPolicy, ClaimsValidation, TokenValidator};
use moka::future::Cache;
use reqwest::Client;

use crate::sources::{Source, file::FileSource, null::NullSource};
use crate::{
    config::{ApplicationConfig, DataSourceKind},
    errors::ProviderError,
};

const CACHE_MAX_CAPACITY: u64 = 1000;

pub struct AppState {
    pub config: ApplicationConfig,
    pub source: Box<dyn Source + Sync + Send + 'static>,
    pub validator: TokenValidator,
}

impl AppState {
    pub fn from_config() -> Result<Self, ProviderError> {
        let config = ApplicationConfig::from_sources()?;
        dbg!(&config);
        let source: Box<dyn Source + Sync + Send + 'static> = match config.source_kind {
            DataSourceKind::Null => Box::new(NullSource),
            DataSourceKind::File => {
                let path = PathBuf::from(&config.source_location);
                Box::new(FileSource::from_path(&path)?)
            }
            DataSourceKind::Database => todo!(),
        };
        let validator = mk_validator(&config);
        Ok(Self {
            config,
            source,
            validator,
        })
    }
}

fn mk_validator(config: &ApplicationConfig) -> TokenValidator {
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
