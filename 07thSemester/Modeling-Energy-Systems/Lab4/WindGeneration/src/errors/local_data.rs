use thiserror::Error;

#[derive(Error, Debug)]
pub enum LocalDataError {
    #[error("Error occurred while reading JSON file.")]
    ReadError(String),

    #[error("Error occurred while deserializing JSON.")]
    ParsingError(String),

    #[error("Latitude or Longitude conversion failed.")]
    ConversionToFloatError,

    #[error("There's no temperatures in JSON structure")]
    UndefinedTemperatureStructure,

    #[error("There's no temperatures in JSON structure")]
    UndefinedWindSpeedStructure,
}
