use crate::config::Config;
use crate::errors::ProjectError;
use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, o2o)]
#[o2o(map_owned(log::LevelFilter))]
pub enum LogLevel {
    #[default]
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

pub struct Logger {
    log_level: LevelFilter,
}

impl Logger {
    pub fn from_config(config: &Config) -> Self {
        Self {
            log_level: config.log_level.into(),
        }
    }

    pub fn setup(self) -> Result<(), ProjectError> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let file_name = Self::generate_file_name();
        let path = Self::path(file_name)?;

        let file = fern::log_file(path).map_err(LoggerError::IO)?;

        fern::Dispatch::new()
            .level(self.log_level)
            .format(move |out, message, record| {
                let time = Local::now();
                out.finish(format_args!(
                    "[{:0>2}-{:0>2}-{:0>2} {:0>2}:{:0>2} {}] {}",
                    time.year(),
                    time.month(),
                    time.day(),
                    time.hour(),
                    time.minute(),
                    record.level(),
                    message
                ))
            })
            .chain(file)
            .apply()
            .map_err(LoggerError::SetLoggerError)
            .map_err(ProjectError::Logger)
    }

    fn generate_file_name() -> String {
        let now = Local::now();
        let date = format!(
            "{year:04}-{day:02}-{month:02}",
            year = now.year(),
            day = now.day(),
            month = now.month(),
        );

        format!("{date}.log")
    }

    pub fn path(file_name: String) -> Result<PathBuf, ProjectError> {
        const LOG_DIR: &str = "logs";
        let mut current_dir = std::env::current_exe().map_err(LoggerError::IO)?;
        current_dir.pop(); // Remove executable name
        current_dir.push(LOG_DIR);

        std::fs::create_dir_all(&current_dir).map_err(LoggerError::IO)?;

        Ok(current_dir.join(file_name))
    }
}

#[derive(Debug, Error)]
pub enum LoggerError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Set Logger: {0}")]
    SetLoggerError(#[from] log::SetLoggerError),
}
