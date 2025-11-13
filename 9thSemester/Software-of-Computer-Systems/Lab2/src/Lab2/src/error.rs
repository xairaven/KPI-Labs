use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Type: I/O. {0}")]
    IO(IOError),

    #[error("Type: Logging. {0}")]
    Log(LogError),
}

#[derive(Debug, Error)]
pub enum IOError {
    #[error("Code file not found. {0}")]
    CodeFileNotFound(io::Error),

    #[error("Failed to read code file. {0}")]
    FailedToReadCodeFile(io::Error),
}

#[derive(Error, Debug)]
pub enum LogError {
    #[error("IO Error. {0}")]
    IO(#[from] io::Error),

    #[error("Logger initialization error. {0}")]
    SetLogger(log::SetLoggerError),
}
