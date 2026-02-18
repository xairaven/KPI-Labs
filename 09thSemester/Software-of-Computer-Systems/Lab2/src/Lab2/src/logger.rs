use crate::error::{Error, LogError};
use log::{LevelFilter, Record};
use std::fmt::Arguments;
use std::path::PathBuf;

pub const DEFAULT_FORMAT: &str = "%MESSAGE";

pub struct LogSettings {
    pub level: LevelFilter,
    pub output_file: Option<PathBuf>,
}

impl Default for LogSettings {
    fn default() -> Self {
        Self {
            level: LevelFilter::Warn,
            output_file: None,
        }
    }
}

impl LogSettings {
    pub fn with_level(self, level: LevelFilter) -> Self {
        Self {
            level,
            output_file: self.output_file,
        }
    }

    pub fn with_output_file(self, output_file: Option<PathBuf>) -> Self {
        Self {
            level: self.level,
            output_file,
        }
    }

    pub fn setup(&self) -> Result<(), Error> {
        if self.level.eq(&LevelFilter::Off) {
            return Ok(());
        }

        let mut dispatcher = fern::Dispatch::new().level(self.level).format({
            move |out, message, record| {
                let formatted = Self::parse_format(DEFAULT_FORMAT, message, record);

                out.finish(format_args!("{formatted}"))
            }
        });

        let output_destination = match &self.output_file {
            Some(path) => OutputDestination::File(path.clone()),
            None => OutputDestination::Console,
        };

        match output_destination {
            OutputDestination::Console => {
                dispatcher = dispatcher.chain(std::io::stdout());
            },
            OutputDestination::File(path) => {
                let file = fern::log_file(&path)
                    .map_err(LogError::IO)
                    .map_err(Error::Log)?;
                dispatcher = dispatcher.chain(file);
            },
        }

        dispatcher
            .apply()
            .map_err(LogError::SetLogger)
            .map_err(Error::Log)
    }

    fn parse_format(_: &str, message: &Arguments, _: &Record) -> String {
        message.to_string()
    }
}

pub enum OutputDestination {
    Console,
    File(PathBuf),
}
