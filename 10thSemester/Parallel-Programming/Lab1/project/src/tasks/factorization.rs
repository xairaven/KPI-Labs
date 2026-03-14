use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

pub struct Factorization {
    target_number: u64,
}

impl Default for Factorization {
    fn default() -> Self {
        Self {
            // This is a product of two large primes (4294967291 * 4294967279).
            // It will force the CPU to iterate up to the square root.
            target_number: 18_446_743_979_220_271_189,
        }
    }
}

impl Factorization {
    // Core CPU-bound algorithm for a specific range of potential divisors.
    fn check_range(n: u64, start: u64, end: u64) -> Option<u64> {
        // Force the starting point to be odd if it's greater than 2.
        let mut current = if start.is_multiple_of(2) && start > 2 {
            start + 1
        } else {
            start
        };

        // Handle the only even prime number separately.
        if start <= 2 && 2 <= end {
            if n.is_multiple_of(current) {
                return Some(2);
            }
            current = 3;
        }

        // Check odd numbers in the assigned chunk.
        while current <= end {
            if n.is_multiple_of(current) {
                return Some(current);
            }
            current += 2;
        }

        None
    }

    fn limit(&self) -> u64 {
        // Calculate the upper bound for our search space.
        (self.target_number as f64).sqrt() as u64
    }
}

impl Benchmarkable for Factorization {}
impl Manageable for Factorization {}
impl Measurable for Factorization {}

impl Reportable for Factorization {
    fn name(&self) -> &'static str {
        "CPU-Bound: Factorization"
    }
}

impl Executable for Factorization {
    fn run_sequential(&self) -> Result<(), Error> {
        // let target = self.target_number;
        let target = std::hint::black_box(self.target_number);

        let limit = self.limit();
        let result = Self::check_range(target, 2, limit);

        let result = match result {
            Some(factor) => factor.to_string(),
            None => "None".to_string(),
        };

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            result
        );

        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        let limit = self.limit();
        let chunk_size = limit.div_ceil(threads as u64);

        let result = AtomicU64::new(0);

        // Spawn scoped threads to partition the search space.
        thread::scope(|s| {
            for i in 0..threads {
                let target = self.target_number;
                let start = std::cmp::max(2, (i as u64) * chunk_size);
                let end = std::cmp::min(limit, start + chunk_size - 1);

                let result = &result;

                s.spawn(move || {
                    if start <= end
                        && let Some(target) = Self::check_range(target, start, end)
                    {
                        // Attempt to set the result if it's not already set by another thread.
                        result
                            .compare_exchange(
                                0,
                                target,
                                Ordering::SeqCst,
                                Ordering::SeqCst,
                            )
                            .ok();
                    }
                });
            }
        });

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            result.load(Ordering::SeqCst)
        );

        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        let limit = self.limit();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::RayonPoolBuild)?;

        // Divide the workload into small chunks for rayon to distribute.
        let chunks_count = workers * 10;
        let chunk_size = limit.div_ceil(chunks_count as u64);

        let target = self.target_number;

        // Execute the parallel iterator within our custom thread pool.
        let result = pool.install(|| {
            use rayon::prelude::*;

            (0..chunks_count).into_par_iter().find_map_any(|i| {
                let start = std::cmp::max(2, (i as u64) * chunk_size);
                let end = std::cmp::min(limit, start + chunk_size - 1);

                if start <= end {
                    Self::check_range(target, start, end)
                } else {
                    None
                }
            })
        });

        let result = match result {
            Some(factor) => factor.to_string(),
            None => "None".to_string(),
        };

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            result
        );

        Ok(())
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        let limit = self.limit();
        let chunk_size = limit.div_ceil(total_processes as u64);

        let start = std::cmp::max(2, (process_index as u64) * chunk_size);
        let end = std::cmp::min(limit, start + chunk_size - 1);

        let mut found_factor = 0;

        // Execute only the mathematical range assigned to this specific process.
        if start <= end
            && let Some(result) = Self::check_range(self.target_number, start, end)
        {
            found_factor = result;
        }

        // Print the result to stdout so the orchestrator can capture it.
        println!("{}", found_factor);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut final_factor: Option<u64> = None;

        // Parse the vector of strings collected from all child processes.
        for result_str in results {
            match result_str.trim().parse::<u64>() {
                Ok(factor) => {
                    // If any process found a factor greater than 0, we record it.
                    if factor > 0 {
                        if let Some(existing) = final_factor {
                            // If multiple processes found factors, we take the smallest one.
                            final_factor = Some(existing.min(factor));
                        } else {
                            final_factor = Some(factor);
                        }
                    }
                },
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        // Log the aggregated final result from all processes.
        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            final_factor
                .map(|factor| factor.to_string())
                .unwrap_or("None".to_string()),
        );

        Ok(())
    }
}
