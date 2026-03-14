use crate::errors::Error;
use crate::task;
use crate::task::measure::Measurable;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq)]
pub enum BenchmarkKind {
    Sequential,
    Threads,
    Workers,
    Processes,
}

#[derive(Debug, Clone)]
pub enum BenchmarkMetadata {
    Sequential,
    Threads(usize),
    Workers(usize),
    Processes(usize),
}

impl BenchmarkMetadata {
    pub fn kind(&self) -> BenchmarkKind {
        match self {
            Self::Sequential => BenchmarkKind::Sequential,
            Self::Threads(_) => BenchmarkKind::Threads,
            Self::Workers(_) => BenchmarkKind::Workers,
            Self::Processes(_) => BenchmarkKind::Processes,
        }
    }
}

impl std::fmt::Display for BenchmarkKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Sequential => "Sequential",
            Self::Threads => "Multi-thread",
            Self::Workers => "Worker pool",
            Self::Processes => "Multi-process",
        };
        write!(f, "{}", str)
    }
}

impl std::fmt::Display for BenchmarkMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Sequential => "Sequential".to_string(),
            Self::Threads(units) => format!("{} threads", units),
            Self::Workers(units) => format!("{} workers", units),
            Self::Processes(units) => format!("{} processes", units),
        };

        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub metadata: BenchmarkMetadata,
    pub duration: Duration,
}

impl BenchmarkResult {
    pub fn filter_by_kind(benchmarks: &[Self], kind: &BenchmarkKind) -> Vec<Self> {
        benchmarks
            .iter()
            .filter(|result| result.metadata.kind().eq(kind))
            .cloned()
            .collect()
    }
}

impl std::fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {:.4} seconds",
            self.metadata,
            self.duration.as_secs_f64()
        )
    }
}

pub trait Benchmarkable: Measurable {
    fn benchmark_tasks(&self) -> Vec<BenchmarkMetadata> {
        task::BENCHMARKS.to_vec()
    }

    fn benchmark(&mut self, task_index: usize) -> Result<Vec<BenchmarkResult>, Error> {
        let mut results = Vec::new();
        for task_metadata in self.benchmark_tasks() {
            match task_metadata {
                BenchmarkMetadata::Sequential => {
                    let duration = self.measure_sequential()?;
                    results.push(BenchmarkResult {
                        metadata: BenchmarkMetadata::Sequential,
                        duration,
                    });
                },
                BenchmarkMetadata::Threads(units) => {
                    let duration = self.measure_threads(units)?;
                    results.push(BenchmarkResult {
                        metadata: BenchmarkMetadata::Threads(units),
                        duration,
                    });
                },
                BenchmarkMetadata::Workers(units) => {
                    let duration = self.measure_workers(units)?;
                    results.push(BenchmarkResult {
                        metadata: BenchmarkMetadata::Workers(units),
                        duration,
                    });
                },
                BenchmarkMetadata::Processes(units) => {
                    let duration = self.measure_processes(task_index, units)?;
                    results.push(BenchmarkResult {
                        metadata: BenchmarkMetadata::Processes(units),
                        duration,
                    });
                },
            }
        }
        Ok(results)
    }
}
