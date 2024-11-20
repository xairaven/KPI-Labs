use crate::context::Context;
use crate::error::LabError;
use std::fs;

pub fn parse(file_name: &str) -> Result<Context, LabError> {
    let json_content =
        fs::read_to_string(file_name).map_err(|err| LabError::ReadFailed(err.to_string()))?;

    serde_json::from_str(&json_content).map_err(|err| LabError::ParseFailed(err.to_string()))
}
