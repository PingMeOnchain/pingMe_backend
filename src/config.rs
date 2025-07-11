use dotenv::dotenv;
use envy;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub database_url: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing or invalid environment variable: {0}")]
    MissingVar(String),
    #[error(transparent)]
    Envy(#[from] envy::Error),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();
        let config = envy::from_env::<Config>().map_err(|e| match e {
            envy::Error::MissingValue(field) => ConfigError::MissingVar(field.to_string()),
            other => ConfigError::Envy(other),
        })?;
        Ok(config)
    }
}
