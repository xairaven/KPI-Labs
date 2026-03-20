use crate::cli::{CliError, InputArgs};
use crate::errors::Error;
use crate::task::report::Reportable;
use crate::tasks::array::ArrayStats;
use crate::tasks::html::HtmlTags;
use crate::tasks::multiplication::MatrixMultiplication;
use crate::tasks::video::VideoPipeline;
use thiserror::Error;

pub struct BenchmarkRunner {
    tasks: Vec<Box<dyn Reportable>>,
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self {
            tasks: vec![
                Box::new(HtmlTags::with_documents_amount(10_000)),
                Box::new(ArrayStats::with_size(10_000_000)),
                Box::new(MatrixMultiplication::with_size(1_000)),
                Box::new(VideoPipeline::with_video("./Nature.webm")),
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
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

pub mod array;
pub mod html;
pub mod multiplication;
pub mod video;
