use std::path::PathBuf;

use crate::sources::{Source, file::FileSource, null::NullSource};
use crate::{
    config::{ApplicationConfig, DataSourceKind},
    errors::ProviderError,
};

pub struct AppState {
    pub config: ApplicationConfig,
    pub source: Box<dyn Source + Sync + Send + 'static>,
}

impl AppState {
    pub fn from_config() -> Result<Self, ProviderError> {
        let config = ApplicationConfig::from_sources()?;
        let source: Box<dyn Source + Sync + Send + 'static> = match config.source_kind {
            DataSourceKind::Null => Box::new(NullSource),
            DataSourceKind::File => {
                let path = PathBuf::from(&config.source_location);
                Box::new(FileSource::from_path(&path)?)
            }
            DataSourceKind::Database => todo!(),
        };
        Ok(Self { config, source })
    }
}
