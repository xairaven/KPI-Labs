use crate::geometry::point2d::Point2D;
use crate::interpolation::state::InterpolationContext;
use std::fs;
use std::path::PathBuf;
use thiserror::Error;

pub fn load_with_file_pick(ctx: &mut InterpolationContext) -> Result<(), FileError> {
    if let Some(path) = rfd::FileDialog::new().pick_file() {
        load_from_path(ctx, path)?
    }

    Ok(())
}

pub fn load_from_path(
    ctx: &mut InterpolationContext, path: PathBuf,
) -> Result<(), FileError> {
    let text = fs::read_to_string(&path)?;

    let deserialized: Vec<Point2D> = serde_json::from_str(&text)?;

    ctx.points_view = deserialized;
    ctx.initialize();

    Ok(())
}

pub fn save_with_file_pick(ctx: &mut InterpolationContext) -> Result<(), FileError> {
    let filter = FileFilter::json();

    if let Some(path) = rfd::FileDialog::new()
        .add_filter(filter.name, &filter.file_extensions)
        .save_file()
    {
        save_to_path(ctx, path)?;
    }

    Ok(())
}

pub fn save_to_path(
    ctx: &mut InterpolationContext, path: PathBuf,
) -> Result<(), FileError> {
    let serialized = serde_json::to_string(&ctx.points_view)?;
    fs::write(path, serialized).map_err(FileError::Io)
}

#[derive(Error, Debug)]
pub enum FileError {
    #[error("Input/output error.")]
    Io(#[from] std::io::Error),

    #[error("Serialization error.")]
    Json(#[from] serde_json::Error),
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
