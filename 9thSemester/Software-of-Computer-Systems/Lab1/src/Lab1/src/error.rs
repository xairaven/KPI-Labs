use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Type: I/O. {0}")]
    IO(IOError),
}

#[derive(Debug, Error)]
pub enum IOError {
    #[error("Code file not found. {0}")]
    CodeFileNotFound(io::Error),

    #[error("Failed to read code file. {0}")]
    FailedToReadCodeFile(io::Error),

    #[error("Failed to write into output file. {0}")]
    FailedToWriteIntoOutputFile(io::Error),
}
