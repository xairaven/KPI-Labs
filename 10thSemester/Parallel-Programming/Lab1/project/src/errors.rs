use crate::logs::LogError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("User input. {0}")]
    Cli(#[from] CliError),

    #[error("Setup failed. {0}")]
    Setup(#[from] SetupError),

    #[error("Teardown failed. {0}")]
    Teardown(#[from] TeardownError),

    #[error("System. {0}")]
    System(#[from] SystemError),

    #[error("Task execution. {0}")]
    Task(#[from] TaskError),

    #[error("Logging. {0}")]
    Log(#[from] LogError),
}

#[derive(Debug, thiserror::Error)]
pub enum CliError {
    #[error("Invalid task number - {0}. Valid task numbers are 1-5.")]
    InvalidTaskNumber(usize),

    #[error("Process index is required when processes mode is enabled.")]
    MissingProcessIndex,

    #[error("Total processes is required when processes mode is enabled.")]
    MissingTotalProcesses,
}

#[derive(Debug, thiserror::Error)]
pub enum SetupError {
    #[error("Failed to create a folder. {0}")]
    CreateFolder(std::io::Error),

    #[error("Failed to create a file. {0}")]
    FileCreation(std::io::Error),

    #[error("Failed to write to a file. {0}")]
    FileWrite(std::io::Error),

    #[error("Failed to create temporary directory. {0}")]
    TemporaryDirectoryCreate(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TeardownError {
    #[error("Failed to delete temporary directory. {0}")]
    TemporaryDirectoryDelete(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum SystemError {
    #[error("Failed to get current executable path. {0}")]
    CurrentExe(std::io::Error),

    #[error("Failed to spawn child process. {0}")]
    ChildProcess(std::io::Error),

    #[error("Failed to read directory contents. {0}")]
    DirectoryRead(std::io::Error),

    #[error("Expected a directory but found a file at path: {0}")]
    DirectoryExpected(String),

    #[error("Failed to read a directory entry. {0}")]
    DirectoryEntry(std::io::Error),

    #[error("Failed to parse child process output - \"{0}\"")]
    FailedParsingChildOutput(String),

    #[error("Failed to open a file. {0}")]
    FileOpen(std::io::Error),

    #[error("Failed to read a file. {0}")]
    FileRead(std::io::Error),

    #[error("Failed to build a rayon worker pool. {0}")]
    RayonPoolBuild(#[from] rayon::ThreadPoolBuildError),

    #[error("Thread panicked during execution.")]
    ThreadPanicked,

    #[error("Failed while waiting for child process to finish. {0}")]
    WaitingForChildProcess(std::io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TaskError {
    #[error("Indexing out of bounds. Tried to access index {0}.")]
    IndexingOutOfBounds(usize),

    #[error("Indexing out of bounds. Tried to access range {0}..{1}")]
    ChunkOutOfBounds(usize, usize),
}
