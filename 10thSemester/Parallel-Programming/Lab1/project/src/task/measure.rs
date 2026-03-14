use crate::errors::Error;
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use std::time::{Duration, Instant};

pub trait Measurable: Executable + Manageable {
    // Measures the exact execution time of the sequential approach
    fn measure_sequential(&mut self) -> Result<Duration, Error> {
        self.setup()?;
        let start = Instant::now();
        self.run_sequential()?;
        let elapsed = start.elapsed();
        self.teardown()?;
        Ok(elapsed)
    }

    // Measures the execution time of the thread-based approach
    fn measure_threads(&mut self, num_threads: usize) -> Result<Duration, Error> {
        self.setup()?;
        let start = Instant::now();
        self.run_threads(num_threads)?;
        let elapsed = start.elapsed();
        self.teardown()?;
        Ok(elapsed)
    }

    // Measures the execution time of the worker-based approach
    fn measure_workers(&mut self, num_workers: usize) -> Result<Duration, Error> {
        self.setup()?;
        let start = Instant::now();
        self.run_workers(num_workers)?;
        let elapsed = start.elapsed();
        self.teardown()?;
        Ok(elapsed)
    }

    // Measures the execution time of the process-based approach
    fn measure_processes(
        &mut self, task_index: usize, num_processes: usize,
    ) -> Result<Duration, Error> {
        self.setup()?;
        let start = Instant::now();
        self.run_processes(task_index, num_processes)?;
        let elapsed = start.elapsed();
        self.teardown()?;
        Ok(elapsed)
    }
}
