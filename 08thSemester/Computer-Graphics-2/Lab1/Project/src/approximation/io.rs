use crate::approximation::math::MathError;
use crate::approximation::state::ApproximationState;
use crate::geometry::point2d::Point2D;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

pub fn load_with_file_pick(state: &mut ApproximationState) -> Result<(), FileError> {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        load_from_path(state, path)?
    }

    Ok(())
}

pub fn load_from_path(
    state: &mut ApproximationState, path: PathBuf,
) -> Result<(), FileError> {
    let text = fs::read_to_string(&path)?;

    let deserialized: Vec<Point2D> = serde_json::from_str(&text)?;

    state.points_view = deserialized;
    state.initialize()?;

    Ok(())
}

pub fn save_with_file_pick(state: &mut ApproximationState) -> Result<(), FileError> {
    let filter = FileFilter::json();

    if let Some(path) = rfd::FileDialog::new()
        .add_filter(filter.name, &filter.file_extensions)
        .save_file()
    {
        save_to_path(state, path)?;
    }

    Ok(())
}

pub fn save_to_path(
    state: &mut ApproximationState, path: PathBuf,
) -> Result<(), FileError> {
    let serialized = serde_json::to_string(&state.points_view)?;
    fs::write(path, serialized).map_err(FileError::Io)
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Input/output error.")]
    Io(#[from] std::io::Error),

    #[error("Serialization error.")]
    Json(#[from] serde_json::Error),

    #[error("Math error.")]
    Initialization(#[from] MathError),
}

#[derive(Default)]
pub struct FileFilter {
    pub name: String,
    pub file_extensions: Vec<&'static str>,
}

impl FileFilter {
    pub fn json() -> Self {
        FileFilter {
            name: String::from("JSON"),
            file_extensions: vec!["json"],
        }
    }
}
