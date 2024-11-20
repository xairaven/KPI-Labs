use crate::errors::api::ApiError;

pub fn get(url: String) -> Result<String, ApiError> {
    let response =
        reqwest::blocking::get(&url).map_err(|err| ApiError::RequestError(err.to_string()))?;

    let body = response
        .text()
        .map_err(|err| ApiError::DecodeError(err.to_string()))?;

    Ok(body)
}
