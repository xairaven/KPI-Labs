use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use crate::tasks::TaskLogicError;
use memmap2::MmapMut;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::process::{Command, Stdio};
use std::time::Duration;

#[derive(Debug)]
pub struct InterProcessCommunication {
    random_number: f64,
}

const SHARED_MEMORY_FILE_NAME: &str = "shm.dat";
const PATH_TO_CSHARP_PROJECT: &str = "./Lab3/CSharpWorker/CSharpWorker.csproj";

impl Default for InterProcessCommunication {
    fn default() -> Self {
        let random_number = rand::random::<f64>();

        Self { random_number }
    }
}

impl Reportable for InterProcessCommunication {
    fn name(&self) -> &'static str {
        "Inter-Process Communication"
    }
}

impl Measurable for InterProcessCommunication {}

impl Benchmarkable for InterProcessCommunication {
    fn benchmarks(&self) -> Vec<BenchmarkKind> {
        vec![
            BenchmarkKind::IpcSharedMemory,
            BenchmarkKind::IpcPipes,
            BenchmarkKind::IpcTcp,
        ]
    }
}

impl Manageable for InterProcessCommunication {
    fn setup(&mut self) -> Result<(), Error> {
        log::info!("Building C# Worker project in Release mode...");

        let status = Command::new("dotnet")
            .args(["build", "-c", "Release", PATH_TO_CSHARP_PROJECT])
            .status()
            .map_err(SystemError::ChildProcess)?;

        if !status.success() {
            return Err(TaskLogicError::ChildProcessStatus(status.to_string()))?;
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        // Cleaning up the temporary shared memory file
        let _ = std::fs::remove_file(SHARED_MEMORY_FILE_NAME);
        Ok(())
    }
}

impl Executable for InterProcessCommunication {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![RunMode::IpcSharedMemory, RunMode::IpcPipes, RunMode::IpcTcp]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        match kind {
            BenchmarkKind::IpcSharedMemory => self.run_shared_memory()?,
            BenchmarkKind::IpcPipes => self.run_pipes()?,
            BenchmarkKind::IpcTcp => self.run_tcp()?,
            _ => unreachable!(),
        };

        Ok(())
    }
}

impl InterProcessCommunication {
    fn run_pipes(&self) -> Result<(), Error> {
        let mut child = Command::new("dotnet")
            .args([
                "run",
                "--project",
                PATH_TO_CSHARP_PROJECT,
                "-c",
                "Release",
                "--no-build",
                "--",
                "pipe",
            ])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .map_err(SystemError::ChildProcess)?;

        let mut stdin = child.stdin.take().ok_or(SystemError::StdIn)?;
        let stdout = child.stdout.take().ok_or(SystemError::StdOut)?;

        // Sending the generated number to the C# process
        writeln!(stdin, "{}", self.random_number).map_err(SystemError::Pipe)?;
        drop(stdin);

        // Reading the response back from the C# process
        let mut reader = BufReader::new(stdout);
        let mut response = String::new();
        reader
            .read_line(&mut response)
            .map_err(TaskLogicError::ChildAnswer)?;

        log::info!(
            "[Pipes] Rust sent: {:.4}, Received back: {}",
            self.random_number,
            response.trim()
        );

        let _ = child.wait();
        Ok(())
    }

    fn run_tcp(&self) -> Result<(), Error> {
        let listener = TcpListener::bind("127.0.0.1:0").map_err(SystemError::TcpBind)?;
        let port = listener
            .local_addr()
            .map_err(SystemError::TcpLocalAddr)?
            .port();

        let mut child = Command::new("dotnet")
            .args([
                "run",
                "--project",
                PATH_TO_CSHARP_PROJECT,
                "-c",
                "Release",
                "--no-build",
                "--",
                "tcp",
                &port.to_string(),
            ])
            .spawn()
            .map_err(SystemError::ChildProcess)?;

        // Accepting the connection from the C# worker
        let (mut stream, _) = listener.accept().map_err(SystemError::TcpConnection)?;

        let bytes_to_send = self.random_number.to_ne_bytes();
        stream
            .write_all(&bytes_to_send)
            .map_err(SystemError::BufferWrite)?;

        let mut buffer = [0u8; 8];
        stream
            .read_exact(&mut buffer)
            .map_err(SystemError::BufferRead)?;

        let received = f64::from_ne_bytes(buffer);
        log::info!(
            "[TCP] Rust sent: {:.4}, Received back: {:.4}",
            self.random_number,
            received
        );

        let _ = child.wait();
        Ok(())
    }

    fn run_shared_memory(&self) -> Result<(), Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(SHARED_MEMORY_FILE_NAME)
            .map_err(SystemError::SharedMemoryFile)?;

        file.set_len(8)
            .map_err(SystemError::SharedMemoryFileLength)?;

        let mut memory_mapped_buffer = unsafe {
            MmapMut::map_mut(&file).map_err(SystemError::SharedMemoryFileObject)?
        };

        // Writing the random f64 number directly into the shared memory region
        memory_mapped_buffer[0..8].copy_from_slice(&self.random_number.to_ne_bytes());

        let mut child = Command::new("dotnet")
            .args([
                "run",
                "--project",
                PATH_TO_CSHARP_PROJECT,
                "-c",
                "Release",
                "--no-build",
                "--",
                "shm",
                SHARED_MEMORY_FILE_NAME,
            ])
            .spawn()
            .map_err(SystemError::ChildProcess)?;

        // Active polling to detect when the C# worker negates the number
        loop {
            let current_value = f64::from_ne_bytes(
                memory_mapped_buffer[0..8]
                    .try_into()
                    .map_err(TaskLogicError::FloatFromMemory)?,
            );
            if current_value < 0.0 {
                log::info!(
                    "[SHM] Rust sent: {:.4}, Received back: {:.4}",
                    self.random_number,
                    current_value
                );
                break;
            }
            std::thread::sleep(Duration::from_millis(1));
        }

        let _ = child.wait();
        Ok(())
    }
}
