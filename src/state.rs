use crate::{config::ApplicationConfig, errors::ProviderError};

pub struct AppState {
    pub config: ApplicationConfig,
}

impl AppState {
    pub fn from_config() -> Result<Self, ProviderError> {
        Ok(Self {
            config: ApplicationConfig::from_sources()?,
        })
    }
}
