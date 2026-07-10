use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProviderError {
    #[error("TMS Provider was unable to read application config file; details: {0}")]
    ApplicationConfigFileError(#[from] std::io::Error),

    #[error("TMS Provider was unable to read configuration sources; details: {0}")]
    ApplicationConfigError(String),
}
