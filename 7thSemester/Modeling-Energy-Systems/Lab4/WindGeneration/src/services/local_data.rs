use crate::errors::local_data::LocalDataError;
use std::fs;

pub fn get(file_name: &str) -> Result<Vec<CityLocalData>, LocalDataError> {
    let file_content =
        fs::read_to_string(file_name).map_err(|err| LocalDataError::ReadError(err.to_string()))?;

    let parsed_json: serde_json::Value = serde_json::from_str(&file_content)
        .map_err(|err| LocalDataError::ParsingError(err.to_string()))?;

    let mut results: Vec<CityLocalData> = Vec::new();

    if let serde_json::Value::Object(city_map) = parsed_json {
        for (city_name, data) in city_map {
            let temperatures = data["temperature"]
                .as_array()
                .ok_or(LocalDataError::UndefinedTemperatureStructure)?;
            let mut result_temperatures: Vec<f64> = Vec::new();
            for value in temperatures {
                result_temperatures.push(
                    value
                        .as_f64()
                        .ok_or(LocalDataError::ConversionToFloatError)?,
                );
            }

            let wind_speeds = data["wind_speed"]
                .as_array()
                .ok_or(LocalDataError::UndefinedWindSpeedStructure)?;
            let mut result_wind_speeds: Vec<f64> = Vec::new();
            for value in wind_speeds {
                result_wind_speeds.push(
                    value
                        .as_f64()
                        .ok_or(LocalDataError::ConversionToFloatError)?,
                );
            }

            results.push(CityLocalData {
                name: city_name,
                average_temperatures: result_temperatures,
                average_wind_speeds: result_wind_speeds,
            });
        }
    } else {
        return Err(LocalDataError::ParsingError(
            "Value is not an object".to_string(),
        ));
    }

    Ok(results)
}

pub struct CityLocalData {
    pub name: String,
    pub average_temperatures: Vec<f64>,
    pub average_wind_speeds: Vec<f64>,
}
