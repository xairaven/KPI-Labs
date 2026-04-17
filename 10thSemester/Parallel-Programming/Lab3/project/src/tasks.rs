use crate::cli::{CliError, InputArgs};
use crate::errors::Error;
use crate::task::report::Reportable;
use crate::tasks::bank::Bank;
use crate::tasks::ipc::InterProcessCommunication;
use std::io;
use thiserror::Error;

pub struct BenchmarkRunner {
    tasks: Vec<Box<dyn Reportable>>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self {
            tasks: vec![
                Box::new(Bank::new(150)),
                Box::new(InterProcessCommunication::default()),
            ],
        }
    }
}

impl BenchmarkRunner {
    pub fn run(&mut self, args: InputArgs) -> Result<String, Error> {
        let task_index = args.task;

        if !(1..=self.tasks.len()).contains(&task_index) {
            Err(CliError::UnknownTask(task_index))?;
        }

        let task = self
            .tasks
            .get_mut(task_index - 1)
            .ok_or(CliError::UnknownTask(task_index))?;

        let report = task.report()?;

        Ok(report.get_text())
    }
}

#[derive(Debug, Error)]
pub enum TaskLogicError {
    #[error("Failed to join thread.")]
    JoinThreadFailed,

    #[error("Mutex poisoned.")]
    MutexPoisoned,

    #[error("Child process status is not success. {0}")]
    ChildProcessStatus(String),

    #[error("Failed to get answer from process. {0}")]
    ChildAnswer(io::Error),

    #[error("Failed to convert bytes (from memory) to float. {0}")]
    FloatFromMemory(core::array::TryFromSliceError),
}

pub mod bank;
pub mod ipc;
