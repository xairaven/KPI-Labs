use crate::errors::{Error, SystemError};
use std::process::{Command, Stdio};

pub trait Executable {
    // Executes the task sequentially in a single main thread
    fn run_sequential(&self) -> Result<(), Error>;

    // Executes the task using std::thread
    fn run_threads(&self, threads: usize) -> Result<(), Error>;

    // Executes the task using a thread pool with work-stealing mechanism (`rayon` crate)
    fn run_workers(&self, workers: usize) -> Result<(), Error>;

    // Executes the task by spawning isolated processes
    fn run_processes(&self, task_index: usize, processes: usize) -> Result<(), Error> {
        // Get the absolute path to the currently running executable
        let exe_path = std::env::current_exe().map_err(SystemError::CurrentExe)?;

        let mut children = Vec::new();

        // Spawn the requested number of child processes
        for i in 0..processes {
            let mut cmd = Command::new(&exe_path);

            // Pass the necessary arguments for the child to identify its chunk
            cmd.arg("--task").arg(task_index.to_string())
                .arg("--processes-mode")
                .arg("--process-index").arg(i.to_string())
                .arg("--total-processes").arg(processes.to_string())
                // Capture the output instead of discarding it
                .stdout(Stdio::piped());

            let child = cmd.spawn().map_err(SystemError::ChildProcess)?;
            children.push(child);
        }

        let mut process_outputs = Vec::new();

        // Wait for all children to finish and collect their printed results
        for child in children {
            let output = child
                .wait_with_output()
                .map_err(SystemError::WaitingForChildProcess)?;

            // Convert the raw bytes from stdout into a UTF-8 string
            let stdout_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !stdout_str.is_empty() {
                process_outputs.push(stdout_str);
            }
        }

        // Pass the collected string data back to the task for specific parsing and aggregation
        self.aggregate_process_results(processes, process_outputs)?;

        Ok(())
    }

    // Parses and combines the results returned by the child processes via stdout
    fn aggregate_process_results(
        &self, _total_processes: usize, _results: Vec<String>,
    ) -> Result<(), Error> {
        // Default empty implementation for tasks that do not need IPC aggregation
        Ok(())
    }

    // Executes a specific chunk of the task (Child execution)
    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error>;
}
