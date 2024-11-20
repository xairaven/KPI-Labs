use thiserror::Error;

#[derive(Error, Debug)]
pub enum LabError {
    #[error("Parsing JSON data failed.")]
    ParseFailed(String),

    #[error("Reading JSON data failed.")]
    ReadFailed(String),
}
