use std::{path::PathBuf, time::Duration};

use jwtiny::{AlgorithmPolicy, ClaimsValidation, TokenValidator};
use moka::future::Cache;
use reqwest::Client;

use crate::{auth::mk_validator, sources::{Source, file::FileSource, null::NullSource}};
use crate::{
    config::{ApplicationConfig, DataSourceKind},
    errors::ProviderError,
};

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
