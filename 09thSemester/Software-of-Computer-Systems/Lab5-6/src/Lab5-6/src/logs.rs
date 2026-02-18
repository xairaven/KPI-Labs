use crate::ui;
use chrono::{Datelike, Local, Timelike};
use log::{LevelFilter, Record};
use std::fmt::Arguments;
use thiserror::Error;

pub const DEFAULT_SETTINGS: DefaultLoggerSettings = DefaultLoggerSettings {
    format: "[$Y-$m-$D $H:$M $LEVEL] $MESSAGE",
    log_level: LevelFilter::Off,
};

pub struct DefaultLoggerSettings {
    pub format: &'static str,
    pub log_level: LevelFilter,
}

pub struct Logger {
    file_title: String,
    format: String,
    log_level: LevelFilter,
}

impl Default for Logger {
    fn default() -> Self {
        Logger {
            file_title: ui::DEFAULT_WINDOW_SETTINGS.project_title.to_string(),
            format: DEFAULT_SETTINGS.format.to_string(),
            log_level: DEFAULT_SETTINGS.log_level,
        }
    }
}

impl Logger {
    pub fn with_file_title(self, file_title: &str) -> Self {
        Self {
            file_title: file_title.to_string(),
            format: self.format,
            log_level: self.log_level,
        }
    }

    pub fn with_format(self, format: &str) -> Self {
        Self {
            file_title: self.file_title,
            format: format.to_string(),
            log_level: self.log_level,
        }
    }

    pub fn with_level(self, log_level: LevelFilter) -> Self {
        Self {
            file_title: self.file_title,
            format: self.format,
            log_level,
        }
    }

    pub fn setup(self) -> Result<(), LogError> {
        if self.log_level == LevelFilter::Off {
            return Ok(());
        }

        let file_name = self.generate_file_name();
        let file = fern::log_file(file_name).map_err(LogError::IO)?;

        fern::Dispatch::new()
            .level(self.log_level)
            .format(move |out, message, record| {
                let message = self.format_message(message, record);

                out.finish(format_args!("{message}"))
            })
            .chain(file)
            .apply()
            .map_err(LogError::AlreadyInitialized)
    }

    fn format_message(&self, message: &Arguments, record: &Record) -> String {
        let log_message = self.format.clone();

        // Time
        let time = Local::now();

        log_message
            // Time
            .replacen("$Y", &format!("{:0>2}", time.year()), 1)
            .replacen("$m", &format!("{:0>2}", time.month()), 1)
            .replacen("$D", &format!("{:0>2}", time.day()), 1)
            .replacen("$H", &format!("{:0>2}", time.hour()), 1)
            .replacen("$M", &format!("{:0>2}", time.minute()), 1)
            .replacen("$S", &format!("{:0>2}", time.second()), 1)
            // Level
            .replacen("$LEVEL", record.level().as_str(), 1)
            // Target
            .replacen("$TARGET", record.target(), 1)
            // Message
            .replacen("$MESSAGE", &message.to_string(), 1)
    }

    fn generate_file_name(&self) -> String {
        let now = Local::now();
        let date = format!(
            "{year:04}-{day:02}-{month:02}",
            year = now.year(),
            day = now.day(),
            month = now.month(),
        );

        let title = self.file_title.trim().replace(" ", "-");
        format!("{title}-{date}.log")
    }
}

#[derive(Debug, Error)]
pub enum LogError {
    #[error("Logger is already initialized: {0}")]
    AlreadyInitialized(#[from] log::SetLoggerError),

    #[error("IO: {0}")]
    IO(std::io::Error),
}
