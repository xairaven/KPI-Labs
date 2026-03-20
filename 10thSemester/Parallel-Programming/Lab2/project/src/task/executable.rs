use crate::errors::Error;
use crate::task::benchmark::BenchmarkKind;

pub trait Executable {
    fn supported_modes(&self) -> Vec<RunMode>;
    fn run(&self, kind: BenchmarkKind) -> Result<(), Error>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum RunMode {
    Sequential,
    MapReduce,
    ForkJoin,
    WorkerPool,
    Pipeline,
    ProducerConsumer,
}

impl std::fmt::Display for RunMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let method = match self {
            Self::Sequential => "Sequential",
            Self::MapReduce => "Map-Reduce",
            Self::ForkJoin => "Fork-Join",
            Self::WorkerPool => "Worker Pool",
            Self::Pipeline => "Pipeline",
            Self::ProducerConsumer => "Producer-Consumer",
        };

        write!(f, "{}", method)
    }
}
