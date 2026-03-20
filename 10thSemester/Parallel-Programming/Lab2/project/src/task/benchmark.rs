use crate::errors::Error;
use crate::task::executable::RunMode;
use crate::task::measure::Measurable;
use std::time::Duration;
use thiserror::Error;

pub trait Benchmarkable: Measurable {
    fn benchmarks(&self) -> Vec<BenchmarkKind>;

    fn run_benchmarks(&mut self) -> Result<Vec<BenchmarkResult>, Error> {
        let mut results = Vec::new();

        // Not benchmarking if there are not supported methods
        for benchmark in &self.benchmarks() {
            let mode = RunMode::from(benchmark);
            if !self.supported_modes().contains(&mode) {
                return Err(Error::Benchmark(BenchmarkError::MethodIsNotSupported(mode)));
            }
        }

        for benchmark in self.benchmarks() {
            let duration = self.measure(benchmark.clone())?;
            let result = BenchmarkResult {
                kind: benchmark,
                time: duration,
            };
            results.push(result);
        }

        Ok(results)
    }
}

#[derive(Debug, Clone)]
pub enum BenchmarkKind {
    Sequential,

    MapReduce {
        partitions: usize,
    },

    ForkJoin {
        threshold: usize,
    },

    WorkerPool {
        workers: usize,
    },

    Pipeline {
        buffer_capacity: usize,
        max_threads: usize,
    },

    ProducerConsumer {
        buffer_capacity: usize,
        consumers: usize,
        producers: usize,
    },
}

pub struct BenchmarkResult {
    pub kind: BenchmarkKind,
    pub time: Duration,
}

#[derive(Debug, Error)]
pub enum BenchmarkError {
    #[error("Method ({0}) is not supported.")]
    MethodIsNotSupported(RunMode),
}

impl From<&BenchmarkKind> for RunMode {
    fn from(kind: &BenchmarkKind) -> Self {
        match kind {
            BenchmarkKind::Sequential => Self::Sequential,
            BenchmarkKind::MapReduce { .. } => Self::MapReduce,
            BenchmarkKind::ForkJoin { .. } => Self::ForkJoin,
            BenchmarkKind::WorkerPool { .. } => Self::WorkerPool,
            BenchmarkKind::Pipeline { .. } => Self::Pipeline,
            BenchmarkKind::ProducerConsumer { .. } => Self::ProducerConsumer,
        }
    }
}

impl std::fmt::Display for BenchmarkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let kind = match self {
            Self::Sequential => String::from("Sequential"),
            Self::MapReduce { partitions } => {
                format!("Map-Reduce with {} partitions", partitions)
            },
            Self::ForkJoin { threshold } => {
                format!("Fork-Join with threshold of {}", threshold)
            },
            Self::WorkerPool { workers } => {
                format!("Worker Pool with {} workers", workers)
            },
            Self::Pipeline {
                buffer_capacity,
                max_threads,
            } => format!(
                "Pipeline with buffer capacity of {} and max {} threads",
                buffer_capacity, max_threads
            ),
            Self::ProducerConsumer {
                buffer_capacity,
                consumers,
                producers,
            } => format!(
                "Producer-Consumer with buffer capacity of {}, {} consumers and {} producers",
                buffer_capacity, consumers, producers
            ),
        };

        write!(f, "{}", kind)
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {:.4} seconds.", self.kind, self.time.as_secs_f64())
    }
}
