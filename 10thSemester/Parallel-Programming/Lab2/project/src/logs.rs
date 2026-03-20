use crate::cli::InputArgs;
use crate::errors::{Error, SystemError};
use chrono::{Datelike, Local, Timelike};
use log::LevelFilter;
use std::env;
use std::path::PathBuf;
use thiserror::Error;

pub struct Logger {
    log_level: LevelFilter,
}

impl Logger {
    pub fn from_args(args: &InputArgs) -> Self {
        Self {
            log_level: args.log_level,
        }
    }

    pub fn setup(self) -> Result<(), Error> {
        if self.log_level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let path = self.log_file()?;
        let file = fern::log_file(path).map_err(LogError::IO)?;

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
            .map_err(LogError::SetLoggerError)
            .map_err(Error::Log)
    }

    fn log_file(&self) -> Result<PathBuf, Error> {
        let file_name = {
            let now = Local::now();
            let date = format!(
                "{year:04}-{day:02}-{month:02}",
                year = now.year(),
                day = now.day(),
                month = now.month(),
            );

            format!("{date}.log")
        };

        let mut current_dir = env::current_exe().map_err(SystemError::CurrentExe)?;
        current_dir.pop(); // Remove executable name
        current_dir.push("logs");

        std::fs::create_dir_all(&current_dir).map_err(LogError::IO)?;

        Ok(current_dir.join(file_name))
    }
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("IO: {0}")]
    IO(#[from] std::io::Error),

    #[error("Set Logger: {0}")]
    SetLoggerError(#[from] log::SetLoggerError),
}
