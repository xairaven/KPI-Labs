use crate::config::ConfigError;
use crate::io::IoError;
use crate::logs::LogError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("I/O. {0}")]
    IO(#[from] IoError),

    #[error("Logger setup. {0}")]
    Log(#[from] LogError),
}
