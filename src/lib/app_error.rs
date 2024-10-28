use thiserror::Error;

#[derive(Debug, Error)]
#[error("...")]
pub enum AppError {
    ConfigError(#[from] ConfigError),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("database init fail")]
    DatabaseInit,
}
