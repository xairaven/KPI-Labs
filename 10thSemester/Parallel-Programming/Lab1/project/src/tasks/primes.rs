use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use std::ops::RangeInclusive;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub struct Primes {
    range: RangeInclusive<u64>,
}

// Builder
impl Primes {
    pub fn with_range(range: RangeInclusive<u64>) -> Self {
        Self { range }
    }
}

impl Primes {
    // Core mathematical algorithm to verify if a single number is prime.
    fn is_prime(n: u64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }

        // Eliminate multiples of 2 and 3 early to save CPU cycles.
        if n.is_multiple_of(2) || n.is_multiple_of(3) {
            return false;
        }

        let mut i = 5;
        // Check odd divisors up to the square root of n.
        while i * i <= n {
            if n.is_multiple_of(i) || n.is_multiple_of(i + 2) {
                return false;
            }
            i += 6;
        }

        true
    }
}

impl Benchmarkable for Primes {}
impl Measurable for Primes {}
impl Manageable for Primes {}

impl Reportable for Primes {
    fn name(&self) -> &'static str {
        "CPU-Bound: Primes"
    }
}

impl Executable for Primes {
    fn run_sequential(&self) -> Result<(), Error> {
        let mut total_primes = 0;
        for n in self.range.clone() {
            if Self::is_prime(n) {
                total_primes += 1;
            }
        }

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            total_primes
        );

        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        // Shared counter to safely accumulate the total primes found by all threads.
        let total_primes = AtomicUsize::new(0);

        thread::scope(|s| {
            for i in 0..threads {
                let shared_total = &total_primes;

                s.spawn(move || {
                    let mut local_count = 0;
                    let mut current = start_bound + i as u64;

                    // Interleaved distribution ensures perfect mathematical load balancing.
                    while current <= end_bound {
                        if Self::is_prime(current) {
                            local_count += 1;
                        }
                        current += threads as u64;
                    }

                    // Add the local result to the global atomic counter.
                    shared_total.fetch_add(local_count, Ordering::Relaxed);
                });
            }
        });

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            total_primes.load(Ordering::SeqCst)
        );

        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::RayonPoolBuild)?;

        let total_primes: usize = pool.install(|| {
            use rayon::prelude::*;

            // Idiomatic Rayon: parallel iterator handles chunks and work-stealing automatically.
            (start_bound..=end_bound)
                .into_par_iter()
                .filter(|&n| Self::is_prime(n))
                .count()
        });

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            total_primes
        );

        Ok(())
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        let start_bound = *self.range.start();
        let end_bound = *self.range.end();

        let mut local_primes = 0;
        let mut current = start_bound + process_index as u64;

        // Execute only the interleaved items assigned to this specific process.
        while current <= end_bound {
            if Self::is_prime(current) {
                local_primes += 1;
            }
            current += total_processes as u64;
        }

        // Print the localized count to stdout for the orchestrator.
        println!("{}", local_primes);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut total_primes = 0;

        // Parse each process output and accumulate the total count.
        for result_str in results {
            match result_str.trim().parse::<usize>() {
                Ok(count) => {
                    total_primes += count;
                },
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            total_primes,
        );

        Ok(())
    }
}
