use crate::logs;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

const FILE_NAME: &str = "config.toml";

#[derive(Debug)]
pub struct Config {
    pub log_format: String,
    pub log_level: LevelFilter,
    pub pretty_output: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_format: logs::DEFAULT_SETTINGS.format.to_string(),
            log_level: logs::DEFAULT_SETTINGS.log_level,
            pretty_output: false,
        }
    }
}

impl Config {
    pub fn from_file() -> Result<Self, ConfigError> {
        match fs::read_to_string(FILE_NAME) {
            Ok(text) => {
                let dto: ConfigDto =
                    toml::from_str(&text).map_err(ConfigError::Deserialization)?;
                Config::try_from(dto)
            },
            Err(_) => {
                let config = Self::default();
                config.save_to_file()?;
                Ok(config)
            },
        }
    }

    pub fn save_to_file(&self) -> Result<(), ConfigError> {
        let dto = ConfigDto::from(self);

        let data = toml::to_string(&dto).map_err(ConfigError::Serialization)?;

        let path = PathBuf::from(FILE_NAME);

        fs::write(path, data).map_err(ConfigError::IO)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigDto {
    pub log_format: String,
    pub log_level: String,
    pub pretty_output: bool,
}

impl TryFrom<ConfigDto> for Config {
    type Error = ConfigError;

    fn try_from(value: ConfigDto) -> Result<Self, Self::Error> {
        Ok(Self {
            log_format: value.log_format,
            log_level: match value.log_level.trim().to_lowercase().as_str() {
                "off" => Ok(LevelFilter::Off),
                "error" => Ok(LevelFilter::Error),
                "warn" => Ok(LevelFilter::Warn),
                "info" => Ok(LevelFilter::Info),
                "debug" => Ok(LevelFilter::Debug),
                "trace" => Ok(LevelFilter::Trace),
                unknown => Err(Self::Error::UnknownLogLevel(unknown.to_string())),
            }?,
            pretty_output: value.pretty_output,
        })
    }
}

impl From<&Config> for ConfigDto {
    fn from(value: &Config) -> Self {
        Self {
            log_format: value.log_format.clone(),
            log_level: value.log_level.to_string(),
            pretty_output: value.pretty_output,
        }
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Deserialization: {0}")]
    Deserialization(#[from] toml::de::Error),

    #[error("Serialization: {0}")]
    Serialization(#[from] toml::ser::Error),

    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Unknown log level: {0}")]
    UnknownLogLevel(String),
}
