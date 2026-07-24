use dotenvy::dotenv;
use std::env;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub refresh_expiration_hours: i64,
    pub client_url: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing environment variable: {0}")]
    MissingVar(#[from] env::VarError),

    #[error("JWT_EXPIRATION_HOURS must be a valid integer")]
    InvalidJwtExpiration(#[from] std::num::ParseIntError),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().ok();

        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            jwt_secret: env::var("JWT_SECRET")?,
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")?.parse()?,
            refresh_expiration_hours: env::var("REFRESH_EXPIRATION_HOURS")?.parse()?,
            client_url: env::var("CLIENT_BASE_URL")?,
        })
    }
}
