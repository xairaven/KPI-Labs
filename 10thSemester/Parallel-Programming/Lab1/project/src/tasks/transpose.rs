use crate::errors::{Error, SystemError, TaskError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use std::thread;

pub struct Transpose {
    size: usize,
    input_matrix: Vec<u64>,
}

// Builder
impl Transpose {
    pub fn with_size(size: usize) -> Self {
        Transpose {
            size,
            input_matrix: Vec::with_capacity(size * size),
        }
    }
}

impl Transpose {
    pub fn matrix_ref(&self) -> &[u64] {
        &self.input_matrix
    }

    pub fn is_transposed(&self, out_matrix: &[u64]) -> Result<bool, TaskError> {
        // Quick O(1) sanity check to ensure the matrix was transposed correctly.
        // We check off-diagonal elements to verify the swap.

        // Row 0, Col N-1 in the output matrix.
        let index = self.size - 1;
        let top_right_out = out_matrix
            .get(index)
            .ok_or(TaskError::IndexingOutOfBounds(index))?;

        // Row N-1, Col 0 in the input matrix.
        let index = (self.size - 1) * self.size;
        let bottom_left_in = self
            .input_matrix
            .get(index)
            .ok_or(TaskError::IndexingOutOfBounds(index))?;

        // Row 1, Col 2 in the output matrix.
        let index = self.size + 2;
        let arbitrary_out = out_matrix
            .get(index)
            .ok_or(TaskError::IndexingOutOfBounds(index))?;

        // Row 2, Col 1 in the input matrix.
        let index = 2 * self.size + 1;
        let arbitrary_in = self
            .input_matrix
            .get(index)
            .ok_or(TaskError::IndexingOutOfBounds(index))?;

        // Result
        let result = top_right_out == bottom_left_in && arbitrary_out == arbitrary_in;
        Ok(result)
    }
}

impl Benchmarkable for Transpose {}
impl Measurable for Transpose {}

impl Reportable for Transpose {
    fn name(&self) -> &'static str {
        "Memory-Bound: Matrix Transpose"
    }
}

impl Manageable for Transpose {
    fn setup(&mut self) -> Result<(), Error> {
        let total_elements = self.size * self.size;
        let mut data = vec![0u64; total_elements];

        // Fill the matrix with DETERMINISTIC data based on indices.
        // This guarantees all processes see the exact same virtual matrix.
        for (i, item) in data.iter_mut().enumerate() {
            *item = (i % 500) as u64;
        }

        self.input_matrix = data;

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        // Clear the input matrix to free memory.
        self.input_matrix.clear();
        Ok(())
    }
}

impl Executable for Transpose {
    fn run_sequential(&self) -> Result<(), Error> {
        let n = self.size;
        let input = self.matrix_ref();
        let mut out = vec![0u64; n * n];

        // Standard sequential transposition.
        for r in 0..n {
            for c in 0..n {
                let out_index = r * n + c;
                let out = out
                    .get_mut(out_index)
                    .ok_or(TaskError::IndexingOutOfBounds(out_index))?;

                let in_index = c * n + r;
                let in_value = input
                    .get(in_index)
                    .ok_or(TaskError::IndexingOutOfBounds(in_index))?;

                *out = *in_value;
            }
        }

        // Ensuring the logic is correct and preventing false positives in benchmarks.
        let result = self.is_transposed(&out)?;

        log::info!(
            "TASK: {}, MODE: {}, IS TRANSPOSED: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            result
        );

        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        let n = self.size;
        let input = self.matrix_ref();
        let mut out = vec![0u64; n * n];

        let chunk_rows = n.div_ceil(threads);
        let chunk_size_elements = chunk_rows * n;

        thread::scope(|s| {
            for (i, chunk) in out.chunks_mut(chunk_size_elements).enumerate() {
                s.spawn(move || {
                    let start_row = i * chunk_rows;
                    let local_rows = chunk.len() / n;

                    for r_local in 0..local_rows {
                        let r_global = start_row + r_local;
                        for c in 0..n {
                            let out_index = r_local * n + c;
                            let in_index = c * n + r_global;

                            let out = chunk
                                .get_mut(out_index)
                                .ok_or(TaskError::IndexingOutOfBounds(out_index));

                            let in_value = input
                                .get(in_index)
                                .ok_or(TaskError::IndexingOutOfBounds(in_index));

                            match (out, in_value) {
                                (Ok(out), Ok(in_value)) => {
                                    *out = *in_value;
                                },
                                (Err(e), _) | (_, Err(e)) => {
                                    log::error!(
                                        "Thread processing row {}: Error - {}",
                                        r_global,
                                        e
                                    );
                                    std::process::exit(1);
                                },
                            }
                        }
                    }
                });
            }
        });

        let result = self.is_transposed(&out)?;

        log::info!(
            "TASK: {}, MODE: {}, IS TRANSPOSED: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            result
        );

        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        let n = self.size;
        let input = self.matrix_ref();
        let mut out = vec![0u64; n * n];

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::RayonPoolBuild)?;

        pool.install(|| {
            use rayon::prelude::*;

            // Process each row in parallel using Rayon's iterators over mutable chunks.
            out.par_chunks_mut(n)
                .enumerate()
                .for_each(|(r, row_slice)| {
                    for c in 0..n {
                        let in_index = c * n + r;
                        let in_value = input
                            .get(in_index)
                            .ok_or(TaskError::IndexingOutOfBounds(in_index));

                        let out = row_slice
                            .get_mut(c)
                            .ok_or(TaskError::IndexingOutOfBounds(c));

                        match (out, in_value) {
                            (Ok(out), Ok(in_value)) => {
                                *out = *in_value;
                            },
                            (Err(e), _) | (_, Err(e)) => {
                                log::error!("Worker processing row {}: Error - {}", r, e);
                                std::process::exit(1);
                            },
                        }
                    }
                });
        });

        // Ensuring the logic is correct and preventing false positives in benchmarks.
        let result = self.is_transposed(&out)?;

        log::info!(
            "TASK: {}, MODE: {}, IS TRANSPOSED: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            result
        );

        Ok(())
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        let n = self.size;

        // Handle process memory isolation deterministically.
        let mut fallback_input;
        let input = if self.input_matrix.is_empty() {
            let total_elements = n * n;
            fallback_input = vec![0u64; total_elements];
            for (i, item) in fallback_input.iter_mut().enumerate() {
                *item = (i % 500) as u64;
            }
            &fallback_input
        } else {
            self.matrix_ref()
        };

        let chunk_rows = n.div_ceil(total_processes);
        let start_row = process_index * chunk_rows;
        let end_row = std::cmp::min(n, start_row + chunk_rows);

        let mut control_value = 0;

        if start_row < end_row {
            let mut out_chunk = vec![0u64; (end_row - start_row) * n];

            for r_local in 0..(end_row - start_row) {
                let r_global = start_row + r_local;
                for c in 0..n {
                    let in_index = c * n + r_global;
                    let out_index = r_local * n + c;

                    let out = out_chunk
                        .get_mut(out_index)
                        .ok_or(TaskError::IndexingOutOfBounds(out_index))?;

                    let in_value = input
                        .get(in_index)
                        .ok_or(TaskError::IndexingOutOfBounds(in_index))?;

                    *out = *in_value;
                }
            }

            std::hint::black_box(&mut out_chunk);
            control_value = out_chunk.first().copied().unwrap_or(0);
        }

        println!("{}", control_value);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut final_control_value = 0;

        for result_str in results {
            match result_str.trim().parse::<u32>() {
                Ok(value) => {
                    // Capture any valid control value to confirm execution.
                    final_control_value = value;
                },
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            final_control_value,
        );

        Ok(())
    }
}
