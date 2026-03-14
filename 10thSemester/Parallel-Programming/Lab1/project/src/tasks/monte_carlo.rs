use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use rand::RngExt;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::thread;

pub struct MonteCarlo {
    iterations: usize,
}

// Builder
impl MonteCarlo {
    pub fn with_iterations(iterations: usize) -> Self {
        Self { iterations }
    }
}

// Core logic
impl MonteCarlo {
    const RANGE: std::ops::RangeInclusive<f64> = -1.0..=1.0;

    // Core mathematical logic for generating random points and counting circle hits.
    // Isolated here to be easily reused across different execution modes.
    fn calculate_chunk(iterations: usize) -> usize {
        let mut rng = rand::rng();
        let mut hits = 0;

        for _ in 0..iterations {
            let x: f64 = rng.random_range(Self::RANGE);
            let y: f64 = rng.random_range(Self::RANGE);

            // Check if the generated point falls inside the unit circle
            if x * x + y * y <= 1.0 {
                hits += 1;
            }
        }

        hits
    }

    // Estimates Pi based on the total hits and total iterations.
    fn calculate_result(&self, total_hits: usize, real_iterations: usize) -> f64 {
        4.0 * (total_hits as f64) / (real_iterations as f64)
    }
}

impl Benchmarkable for MonteCarlo {}
impl Measurable for MonteCarlo {}
impl Manageable for MonteCarlo {}

impl Reportable for MonteCarlo {
    fn name(&self) -> &'static str {
        "CPU-Bound: Monte-Carlo"
    }
}

impl Executable for MonteCarlo {
    fn run_sequential(&self) -> Result<(), Error> {
        let hits = Self::calculate_chunk(self.iterations);
        let result = self.calculate_result(hits, self.iterations);

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            result
        );
        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        let iterations_per_thread = self.iterations / threads;
        let mut handles = Vec::with_capacity(threads);

        for _ in 0..threads {
            handles.push(thread::spawn(move || {
                Self::calculate_chunk(iterations_per_thread)
            }));
        }

        let mut total_hits: usize = 0;
        for handle in handles {
            total_hits += handle.join().map_err(|_| SystemError::ThreadPanicked)?;
        }
        let result = self.calculate_result(total_hits, iterations_per_thread * threads);

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            result
        );
        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        let iterations_per_worker = self.iterations / workers;

        // Use Rayon to create a parallel iterator over the number of workers
        let total_hits: usize = (0..workers)
            .into_par_iter()
            .map(|_| Self::calculate_chunk(iterations_per_worker))
            .sum();

        let result = self.calculate_result(total_hits, iterations_per_worker * workers);

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            result
        );
        Ok(())
    }

    fn run_process_chunk(
        &self, _process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        // Each child process calculates its own chunk independently
        let iterations_per_process = self.iterations / total_processes;
        let hits = Self::calculate_chunk(iterations_per_process);

        println!("{}", hits);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut total_hits = 0;

        // Parse the vector of strings collected from all child processes
        for result_str in results {
            match result_str.trim().parse::<usize>() {
                Ok(hits) => total_hits += hits,
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        let real_iterations = self.iterations / total_processes * total_processes;
        let result = self.calculate_result(total_hits, real_iterations);

        // Assuming the total iterations were divided equally among processes
        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            result,
        );
        Ok(())
    }
}
