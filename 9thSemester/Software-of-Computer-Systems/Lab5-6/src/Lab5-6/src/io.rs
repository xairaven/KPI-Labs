use thiserror::Error;

#[derive(Debug, Error)]
pub enum IoError {
    #[error("Failed to read file: {0}")]
    ReadFile(std::io::Error),
}
