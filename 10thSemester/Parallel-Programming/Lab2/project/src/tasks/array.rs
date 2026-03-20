use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use rand::RngExt;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;

type Number = f64;

#[derive(Debug)]
pub struct ArrayStats {
    pub size: usize,

    numbers: Vec<Number>,
}

// Builder
impl ArrayStats {
    pub fn with_size(size: usize) -> Self {
        Self {
            size,
            numbers: vec![],
        }
    }
}

impl Reportable for ArrayStats {
    fn name(&self) -> &'static str {
        "Min, Max, Mean, Average of the array"
    }
}

impl Benchmarkable for ArrayStats {
    fn benchmarks(&self) -> Vec<BenchmarkKind> {
        vec![
            BenchmarkKind::Sequential,
            BenchmarkKind::MapReduce { partitions: 8 },
            BenchmarkKind::ForkJoin { threshold: 10_000 },
            BenchmarkKind::ForkJoin { threshold: 1_000 },
            BenchmarkKind::WorkerPool { workers: 2 },
            BenchmarkKind::WorkerPool { workers: 4 },
            BenchmarkKind::WorkerPool { workers: 8 },
        ]
    }
}

impl Manageable for ArrayStats {
    fn setup(&mut self) -> Result<(), Error> {
        let mut rng = rand::rng();

        // Generate a massive array of random floats.
        // We reserve capacity to avoid reallocation overhead.
        self.numbers.reserve_exact(self.size);
        for _ in 0..self.size {
            // Generating values between -1000.0 and 1000.0
            self.numbers.push(rng.random_range(-1000.0..1000.0));
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        self.numbers.clear();
        self.numbers.shrink_to_fit();
        Ok(())
    }
}

impl Executable for ArrayStats {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![
            RunMode::Sequential,
            RunMode::MapReduce,
            RunMode::ForkJoin,
            RunMode::WorkerPool,
        ]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        let stats = match kind {
            BenchmarkKind::Sequential => self.sequential(),
            BenchmarkKind::MapReduce { .. } => self.map_reduce(),
            BenchmarkKind::ForkJoin { threshold } => self.fork_join(threshold),
            BenchmarkKind::WorkerPool { workers } => self.workers(workers),
            _ => return Ok(()),
        }?;

        log::info!(
            "TASK: {}, MODE: {}, STATS: Min: {:.2}, Max: {:.2}, Mean: {:.2}, Median: {:.2}",
            self.name(),
            kind,
            stats.min,
            stats.max,
            stats.mean,
            stats.median
        );

        Ok(())
    }
}

impl Measurable for ArrayStats {}

// A helper struct to accumulate our metrics during parallel execution.
#[derive(Debug, Clone, Copy)]
pub struct StatsAccumulator {
    min: Number,
    max: Number,
    sum: Number,
    count: usize,
}

impl StatsAccumulator {
    // Initializes the accumulator with a single value.
    fn from_value(value: Number) -> Self {
        Self {
            min: value,
            max: value,
            sum: value,
            count: 1,
        }
    }

    // Merges two accumulators together, which represents our "Reduce" phase.
    fn merge(self, other: Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
            sum: self.sum + other.sum,
            count: self.count + other.count,
        }
    }
}

// A struct to format the final result for our logs.
#[derive(Debug)]
pub struct FinalStats {
    min: Number,
    max: Number,
    mean: Number,
    median: Number,
}

// Helpers
impl ArrayStats {
    // Recursive Fork-Join implementation for finding min, max, and sum.
    fn fork_join_recursive(data: &[f64], threshold: usize) -> StatsAccumulator {
        // Base case: process sequentially if the chunk is small enough.
        if data.len() <= threshold {
            let mut acc = StatsAccumulator::from_value(data[0]);
            for &val in data.iter().skip(1) {
                acc = acc.merge(StatsAccumulator::from_value(val));
            }
            return acc;
        }

        // Recursive case: split the workload in half.
        let mid = data.len() / 2;
        let (left_data, right_data) = data.split_at(mid);

        // Rayon's join executes closures on potentially different threads.
        let (left_result, right_result) = rayon::join(
            || Self::fork_join_recursive(left_data, threshold),
            || Self::fork_join_recursive(right_data, threshold),
        );

        left_result.merge(right_result)
    }

    // Helper function to calculate the median after sorting.
    fn calculate_median(sorted_data: &[f64]) -> f64 {
        let len = sorted_data.len();
        if len.is_multiple_of(2) {
            (sorted_data[len / 2 - 1] + sorted_data[len / 2]) / 2.0
        } else {
            sorted_data[len / 2]
        }
    }
}

// Modes
impl ArrayStats {
    pub fn sequential(&self) -> Result<FinalStats, Error> {
        // Find min, max, and sum using standard sequential folding.
        let initial = StatsAccumulator::from_value(self.numbers[0]);
        let accumulator = self
            .numbers
            .iter()
            // Because we already initialized with it
            .skip(1)
            .fold(initial, |acc, &val| acc.merge(StatsAccumulator::from_value(val)));

        // Clone and sort sequentially for the median.
        let mut local_copy = self.numbers.clone();
        local_copy.sort_unstable_by(|a, b| a.total_cmp(b));
        let median = Self::calculate_median(&local_copy);

        Ok(FinalStats {
            min: accumulator.min,
            max: accumulator.max,
            mean: accumulator.sum / accumulator.count as f64,
            median,
        })
    }

    pub fn map_reduce(&self) -> Result<FinalStats, Error> {
        // Map-Reduce logic using Rayon's parallel iterators.
        let acc = self
            .numbers
            .par_iter()
            .map(|&val| StatsAccumulator::from_value(val))
            .reduce(
                || StatsAccumulator::from_value(self.numbers[0]),
                |a, b| a.merge(b),
            );

        // Clone and sort in parallel using Rayon.
        let mut local_copy = self.numbers.clone();
        local_copy.par_sort_unstable_by(|a, b| a.total_cmp(b));
        let median = Self::calculate_median(&local_copy);

        Ok(FinalStats {
            min: acc.min,
            max: acc.max,
            mean: acc.sum / acc.count as f64,
            median,
        })
    }

    pub fn fork_join(&self, threshold: usize) -> Result<FinalStats, Error> {
        let acc = Self::fork_join_recursive(&self.numbers, threshold);

        // Utilizing parallel sort for the median to maintain performance.
        let mut local_copy = self.numbers.clone();
        local_copy.par_sort_unstable_by(|a, b| a.total_cmp(b));
        let median = Self::calculate_median(&local_copy);

        Ok(FinalStats {
            min: acc.min,
            max: acc.max,
            mean: acc.sum / acc.count as f64,
            median,
        })
    }

    pub fn workers(&self, workers: usize) -> Result<FinalStats, Error> {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::WorkerPoolBuild)?;

        let final_stats = pool.install(|| {
            let acc = self
                .numbers
                .par_iter()
                .map(|&val| StatsAccumulator::from_value(val))
                .reduce(
                    || StatsAccumulator::from_value(self.numbers[0]),
                    |a, b| a.merge(b),
                );

            let mut local_copy = self.numbers.clone();
            local_copy.par_sort_unstable_by(|a, b| a.total_cmp(b));
            let median = Self::calculate_median(&local_copy);

            FinalStats {
                min: acc.min,
                max: acc.max,
                mean: acc.sum / acc.count as f64,
                median,
            }
        });

        Ok(final_stats)
    }
}
