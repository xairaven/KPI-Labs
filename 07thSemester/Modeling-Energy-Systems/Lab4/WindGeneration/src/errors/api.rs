use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Error occurred during HTTP request.")]
    RequestError(String),

    #[error("Error occurred while decoding HTTP response body.")]
    DecodeError(String),

    #[error("Error occurred while parsing JSON response.")]
    ParsingError(String),

    #[error("There's no location in response array.")]
    UndefinedLocationStructure,

    #[error("Latitude or Longitude conversion failed.")]
    ConversionToFloatError,

    #[error("There's no weather in response array.")]
    UndefinedWeatherStructure,

    #[error("There's no weather_main in response array.")]
    UndefinedWeatherMainStatus,

    #[error("There's no weather_description in response array.")]
    UndefinedWeatherDescription,

    #[error("There's no weather_icon-id in response array.")]
    UndefinedWeatherIconId,

    #[error("There's no main_temperature in response array.")]
    UndefinedTemperature,

    #[error("There's no wind_speed in response array.")]
    UndefinedWindSpeed,

    #[error("There's no wind_degree in response array.")]
    UndefinedWindDegree,
}
