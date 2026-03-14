use crate::task::benchmark::BenchmarkMetadata;
use crate::task::report::Reportable;
use crate::tasks::factorization::Factorization;
use crate::tasks::io::WordCount;
use crate::tasks::monte_carlo::MonteCarlo;
use crate::tasks::primes::Primes;
use crate::tasks::transpose::Transpose;

pub const BENCHMARKS: [BenchmarkMetadata; 10] = [
    BenchmarkMetadata::Sequential,
    BenchmarkMetadata::Threads(2),
    BenchmarkMetadata::Threads(4),
    BenchmarkMetadata::Threads(8),
    BenchmarkMetadata::Workers(2),
    BenchmarkMetadata::Workers(4),
    BenchmarkMetadata::Workers(8),
    BenchmarkMetadata::Processes(2),
    BenchmarkMetadata::Processes(4),
    BenchmarkMetadata::Processes(8),
];

pub fn tasks() -> Vec<Box<dyn Reportable>> {
    vec![
        // CPU-Bound
        Box::new(MonteCarlo::with_iterations(100_000_000)),
        Box::new(Factorization::default()),
        Box::new(Primes::with_range(1..=10_000_000)),
        // Memory-Bound
        Box::new(Transpose::with_size(10_000)),
        // I/O-Bound
        Box::new(WordCount::new(1000, 100)),
    ]
}

pub mod benchmark;
pub mod executor;
pub mod lifecycle;
pub mod measure;
pub mod report;
