use crate::cli::CliError;
use crate::logs::LogError;
use crate::task::benchmark::BenchmarkError;
use crate::tasks::TaskLogicError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Benchmark. {0}")]
    Benchmark(#[from] BenchmarkError),

    #[error("User input. {0}")]
    Cli(#[from] CliError),

    #[error("Logger. {0}")]
    Log(#[from] LogError),

    #[error("System. {0}")]
    System(#[from] SystemError),

    #[error("Task Logic. {0}")]
    TaskLogic(#[from] TaskLogicError),
}

#[derive(Debug, Error)]
pub enum SystemError {
    #[error("Failed to get current executable path. {0}")]
    CurrentExe(io::Error),

    #[error("Failed to create a directory. {0}")]
    DirectoryCreate(io::Error),

    #[error("Failed to read directory content. {0}")]
    DirectoryRead(io::Error),

    #[error("Failed to remove directory. {0}")]
    DirectoryRemove(io::Error),

    #[error("Failed to execute FFmpeg. {0}")]
    FfmpegExecution(io::Error),

    #[error("Failed to fetch file name from the path. {0}")]
    FileName(String),

    #[error("Failed to open image. {0}")]
    ImageOpen(image::ImageError),

    #[error("Failed to save image. {0}")]
    ImageSave(image::ImageError),

    #[error("Failed to build rayon pool. {0}")]
    WorkerPoolBuild(#[from] rayon::ThreadPoolBuildError),
}
