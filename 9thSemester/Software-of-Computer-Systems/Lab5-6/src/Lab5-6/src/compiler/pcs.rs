// Configuration
#[derive(Debug, Default, Clone)]
pub struct SystemConfiguration {
    pub time: TimeConfiguration,
    pub processors: ProcessorConfiguration,
}

#[derive(Debug, Clone)]
pub struct TimeConfiguration {
    pub add: usize,
    pub sub: usize,
    pub mul: usize,
    pub div: usize,
}

impl Default for TimeConfiguration {
    fn default() -> Self {
        Self {
            add: 1,
            sub: 1,
            mul: 2,
            div: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ProcessorConfiguration {
    pub add: usize,
    pub sub: usize,
    pub mul: usize,
    pub div: usize,
}

impl ProcessorConfiguration {
    pub fn total(&self) -> usize {
        self.add + self.sub + self.mul + self.div
    }
}

impl Default for ProcessorConfiguration {
    fn default() -> Self {
        Self {
            add: 1,
            sub: 1,
            mul: 1,
            div: 1,
        }
    }
}

pub mod research;
pub mod vector;
