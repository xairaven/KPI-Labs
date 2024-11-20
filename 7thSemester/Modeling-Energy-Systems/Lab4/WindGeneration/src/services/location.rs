use crate::errors::api::ApiError;
use crate::services::request;

const COUNTRY_CODE: &str = "UA";
const LIMIT: u8 = 1;

pub fn coordinates_by_city_name(api_key: &str, city: &str) -> Result<Location, ApiError> {
    let url = format!(
        "http://api.openweathermap.org/geo/1.0/direct?q={},{}&limit={}&appid={}",
        city, COUNTRY_CODE, LIMIT, api_key
    );

    let body = request::get(url)?;

    let json: serde_json::Value =
        serde_json::from_str(&body).map_err(|err| ApiError::ParsingError(err.to_string()))?;

    let object = json
        .get(0)
        .ok_or(ApiError::UndefinedLocationStructure)?
        .as_object()
        .ok_or(ApiError::UndefinedLocationStructure)?;

    let mut location = Location::default();

    match &object["lat"] {
        serde_json::Value::Number(latitude) => {
            location.latitude = latitude.as_f64().ok_or(ApiError::ConversionToFloatError)?;
        },
        _ => return Err(ApiError::UndefinedLocationStructure),
    }

    match &object["lon"] {
        serde_json::Value::Number(longitude) => {
            location.longitude = longitude.as_f64().ok_or(ApiError::ConversionToFloatError)?;
        },
        _ => return Err(ApiError::UndefinedLocationStructure),
    }

    Ok(location)
}

#[derive(Default)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}
