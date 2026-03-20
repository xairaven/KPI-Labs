use crate::errors::Error;
use crate::task::benchmark::{BenchmarkResult, Benchmarkable};

#[derive(Debug, Default)]
pub struct Report {
    buffer: TextBuffer,
}

impl Report {
    pub fn add_task_header(&mut self, task_name: &str) {
        let header = format!(
            "=== REPORT FOR {} ===",
            task_name.to_ascii_uppercase().trim()
        );
        self.buffer.add_line(&header);
    }

    pub fn add_result(&mut self, result: &BenchmarkResult) {
        self.buffer.add_line(&result.to_string());
    }

    pub fn get_text(&self) -> String {
        self.buffer.text()
    }
}

pub trait Reportable: Benchmarkable {
    fn name(&self) -> &'static str;

    fn report(&mut self) -> Result<Report, Error> {
        let mut report = Report::default();

        report.add_task_header(self.name());

        let results = self.run_benchmarks()?;

        for result in results {
            report.add_result(&result);
        }

        Ok(report)
    }
}

#[derive(Debug, Default)]
pub struct TextBuffer {
    buffer: String,
}

impl TextBuffer {
    pub fn add_line(&mut self, line: &str) {
        self.buffer.push_str(line);
        self.buffer.push('\n');
    }

    pub fn text(&self) -> String {
        self.buffer.clone()
    }
}
