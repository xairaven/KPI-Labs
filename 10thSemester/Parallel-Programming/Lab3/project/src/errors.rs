use crate::cli::CliError;
use crate::logs::LogError;
use crate::task::benchmark::BenchmarkError;
use crate::tasks::TaskLogicError;
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
    #[error("Failed to write into the buffer. {0}")]
    BufferRead(std::io::Error),

    #[error("Failed to write into the buffer. {0}")]
    BufferWrite(std::io::Error),

    #[error("Failed to get current executable path.")]
    CurrentExe(std::io::Error),

    #[error("Failed to create child process. {0}")]
    ChildProcess(std::io::Error),

    #[error("Failed to open stdin.")]
    StdIn,

    #[error("Failed to open stdout.")]
    StdOut,

    #[error("Failed to create shared memory file. {0}")]
    SharedMemoryFile(std::io::Error),

    #[error("Failed to resize shared memory file. {0}")]
    SharedMemoryFileLength(std::io::Error),

    #[error("Failed to create shared memory object. {0}")]
    SharedMemoryFileObject(std::io::Error),

    #[error("Failed to bind socket. {0}")]
    TcpBind(std::io::Error),

    #[error("Failed to get local address of socket. {0}")]
    TcpLocalAddr(std::io::Error),

    #[error("Failed to get connection from the child process. {0}")]
    TcpConnection(std::io::Error),

    #[error("Failed to pass information by pipe. {0}")]
    Pipe(std::io::Error),
}
