use crate::errors::Error;
use crate::task::benchmark::BenchmarkKind;
use crate::task::executable::Executable;
use crate::task::lifecycle::Manageable;
use std::time::{Duration, Instant};

pub trait Measurable: Manageable + Executable {
    fn measure(&mut self, kind: BenchmarkKind) -> Result<Duration, Error> {
        self.setup()?;
        let start = Instant::now();
        self.run(kind)?;
        let elapsed = start.elapsed();
        self.teardown()?;
        Ok(elapsed)
    }
}
