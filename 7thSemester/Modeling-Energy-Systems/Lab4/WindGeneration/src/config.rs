use crate::errors::env::EnvError;
use dotenvy::dotenv;
use std::env;

pub struct Config {
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, EnvError> {
        // Loading .env from crate folder.
        if dotenv().is_err() {
            return Err(EnvError::ConfigNotLoaded);
        }

        // Loading API Key
        let api_key = env::var("API_KEY").map_err(|_| EnvError::APIKeyNotLoaded)?;

        Ok(Self { api_key })
    }
}
