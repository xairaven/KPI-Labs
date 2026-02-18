use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Error occurred while parsing date. Check formats in --help.")]
    DateParse(String),
}
