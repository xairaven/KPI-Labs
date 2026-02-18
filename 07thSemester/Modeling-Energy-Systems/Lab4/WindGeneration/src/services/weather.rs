use crate::errors::api::ApiError;
use crate::services::location::Location;
use crate::services::{location, request};

const UNITS: &str = "metric";

pub fn get(api_key: &str, city: &str) -> Result<Weather, ApiError> {
    let location = location::coordinates_by_city_name(api_key, city)?;

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}&units={}",
        location.latitude, location.longitude, api_key, UNITS
    );

    let body = request::get(url)?;

    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|err| ApiError::ParsingError(err.to_string()))?;

    let weather_data = &json["weather"]
        .get(0)
        .ok_or(ApiError::UndefinedWeatherStructure)?;
    let wind_data = &json["wind"];

    let weather = Weather {
        location,
        main_status: weather_data["main"]
            .as_str()
            .ok_or(ApiError::UndefinedWeatherMainStatus)?
            .to_string(),

        description: weather_data["description"]
            .as_str()
            .ok_or(ApiError::UndefinedWeatherDescription)?
            .to_string(),

        temperature: json["main"]["temp"]
            .as_f64()
            .ok_or(ApiError::UndefinedTemperature)? as f32,

        wind_speed: wind_data["speed"]
            .as_f64()
            .ok_or(ApiError::UndefinedWindSpeed)? as f32,

        wind_direction: wind_data["deg"]
            .as_f64()
            .ok_or(ApiError::UndefinedWindDegree)? as f32,
    };

    Ok(weather)
}

#[derive(Default)]
pub struct Weather {
    pub location: Location,
    pub main_status: String,
    pub description: String,
    pub temperature: f32,
    pub wind_speed: f32,
    pub wind_direction: f32,
}
