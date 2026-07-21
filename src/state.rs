use std::{env, path::PathBuf};

use jwtiny::TokenValidator;
use tracing::{debug, warn};

use crate::{
    auth::mk_validator,
    sources::{Source, file::FileSource, null::NullSource},
};
use crate::{
    config::{ApplicationConfig, DataSourceKind},
    errors::ProviderError,
};

pub struct AppState {
    pub config: ApplicationConfig,
    pub source: Box<dyn Source + Sync + Send + 'static>,
    pub validator: TokenValidator,
    pub version: String,
    pub rust_version: String,
    pub commit: String,
}

const VERSION_VARIABLE: &str = "TMS_PROVIDER_VERSION";
const RUST_VERSION_VARIABLE: &str = "TMS_PROVIDER_RUST_VERSION";
const COMMIT_VARIABLE: &str = "TMS_PROVIDER_COMMIT";

fn get_version() -> String {
    env::var(VERSION_VARIABLE).unwrap_or_else(|err| {
        warn!(
            VERSION_VARIABLE,
            ?err,
            "Environment variable not found; using version 0.0.0"
        );
        "0.0.0".into()
    })
}

fn get_rust_version() -> String {
    env::var(RUST_VERSION_VARIABLE).unwrap_or_else(|err| {
        warn!(
            RUST_VERSION_VARIABLE,
            ?err,
            "Environment variable not found; using Rust version 0.0.0"
        );
        "0.0.0".into()
    })
}

fn get_commit() -> String {
    env::var(COMMIT_VARIABLE).unwrap_or_else(|err| {
        warn!(
            COMMIT_VARIABLE,
            ?err,
            "Environment variable not found; using commit 0000000"
        );
        "0000000".into()
    })
}

impl AppState {
    pub fn from_config() -> Result<Self, ProviderError> {
        let config = ApplicationConfig::from_sources()?;
        debug!(config=?config, "Configuration");
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
            version: get_version(),
            rust_version: get_rust_version(),
            commit: get_commit(),
        })
    }
}
