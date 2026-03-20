use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use rand::RngExt;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Debug)]
pub struct MatrixMultiplication {
    pub size: usize,

    matrix_a: Vec<f64>,
    matrix_b: Vec<f64>,
}

// Builder
impl MatrixMultiplication {
    pub fn with_size(size: usize) -> Self {
        Self {
            size,

            matrix_a: vec![],
            matrix_b: vec![],
        }
    }
}

impl MatrixMultiplication {
    // Pure function to multiply a single row of matrix A with all rows of transposed matrix B.
    // Represents the "Map" phase of our execution.
    fn compute_row(size: usize, row_a: &[f64], matrix_b_transposed: &[f64]) -> Vec<f64> {
        let mut result_row = Vec::with_capacity(size);

        for j in 0..size {
            let mut sum = 0.0;
            // Both row_a and matrix_b_transposed are accessed sequentially here,
            // resulting in perfect CPU cache utilization.
            let row_b = &matrix_b_transposed[j * size..(j + 1) * size];
            for k in 0..size {
                sum += row_a[k] * row_b[k];
            }
            result_row.push(sum);
        }

        result_row
    }

    // Pure function to merge two chunks of matrix rows together.
    // Represents the "Reduce" or "Join" phase.
    fn merge_matrix_chunks(mut chunk_a: Vec<f64>, chunk_b: Vec<f64>) -> Vec<f64> {
        chunk_a.extend(chunk_b);
        chunk_a
    }

    // Transposes the matrix to optimize memory access patterns during multiplication.
    fn transpose(size: usize, matrix: &[f64]) -> Vec<f64> {
        let mut transposed = vec![0.0; size * size];
        for i in 0..size {
            for j in 0..size {
                transposed[j * size + i] = matrix[i * size + j];
            }
        }
        transposed
    }

    // Recursive Fork-Join implementation.
    // We recursively split the workload by dividing the rows of the resulting matrix.
    fn fork_join_recursive(
        size: usize, matrix_a: &[f64], matrix_b_t: &[f64], row_start: usize,
        row_end: usize, threshold: usize,
    ) -> Vec<f64> {
        let row_count = row_end - row_start;

        // Base case: process sequentially if the chunk of rows is small enough.
        if row_count <= threshold {
            let mut result_chunk = Vec::with_capacity(row_count * size);
            for i in row_start..row_end {
                let row_a = &matrix_a[i * size..(i + 1) * size];
                let computed_row = Self::compute_row(size, row_a, matrix_b_t);
                result_chunk.extend(computed_row);
            }
            return result_chunk;
        }

        // Recursive case: split the row range in half.
        let mid = row_start + row_count / 2;

        // Rayon's join executes closures on potentially different threads.
        let (left_result, right_result) = rayon::join(
            || {
                Self::fork_join_recursive(
                    size, matrix_a, matrix_b_t, row_start, mid, threshold,
                )
            },
            || {
                Self::fork_join_recursive(
                    size, matrix_a, matrix_b_t, mid, row_end, threshold,
                )
            },
        );

        Self::merge_matrix_chunks(left_result, right_result)
    }
}

impl Benchmarkable for MatrixMultiplication {
    fn benchmarks(&self) -> Vec<BenchmarkKind> {
        vec![
            BenchmarkKind::Sequential,
            BenchmarkKind::MapReduce { partitions: 8 },
            BenchmarkKind::ForkJoin { threshold: 100 },
            BenchmarkKind::ForkJoin { threshold: 10 },
            BenchmarkKind::WorkerPool { workers: 2 },
            BenchmarkKind::WorkerPool { workers: 4 },
            BenchmarkKind::WorkerPool { workers: 8 },
        ]
    }
}

impl Manageable for MatrixMultiplication {
    fn setup(&mut self) -> Result<(), Error> {
        let mut rng = rand::rng();
        let total_elements = self.size * self.size;

        self.matrix_a.reserve_exact(total_elements);
        self.matrix_b.reserve_exact(total_elements);

        for _ in 0..total_elements {
            self.matrix_a.push(rng.random_range(-10.0..10.0));
            self.matrix_b.push(rng.random_range(-10.0..10.0));
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        self.matrix_a.clear();
        self.matrix_a.shrink_to_fit();
        self.matrix_b.clear();
        self.matrix_b.shrink_to_fit();
        Ok(())
    }
}

impl Executable for MatrixMultiplication {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![
            RunMode::Sequential,
            RunMode::MapReduce,
            RunMode::ForkJoin,
            RunMode::WorkerPool,
        ]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        let result_matrix = match kind {
            BenchmarkKind::Sequential => self.sequential(),
            BenchmarkKind::MapReduce { .. } => self.map_reduce(),
            BenchmarkKind::ForkJoin { threshold } => self.fork_join(threshold),
            BenchmarkKind::WorkerPool { workers } => self.workers(workers),
            _ => return Ok(()),
        }?;

        // A quick validation to ensure the matrix is actually computed.
        // We log the value of the very first element.
        let first_element = result_matrix.first().copied().unwrap_or(0.0);

        log::info!(
            "TASK: {}, MODE: {}, FIRST ELEMENT VALUE: {:.2}",
            self.name(),
            kind,
            first_element
        );

        Ok(())
    }
}

// Modes
impl MatrixMultiplication {
    pub fn sequential(&self) -> Result<Vec<f64>, Error> {
        let matrix_b_t = Self::transpose(self.size, &self.matrix_b);

        // Fold here accumulates rows into the final flat vector.
        let result = (0..self.size)
            .map(|i| {
                let row_a = &self.matrix_a[i * self.size..(i + 1) * self.size];
                Self::compute_row(self.size, row_a, &matrix_b_t)
            })
            .fold(
                Vec::with_capacity(self.size * self.size),
                Self::merge_matrix_chunks,
            );

        Ok(result)
    }

    pub fn map_reduce(&self) -> Result<Vec<f64>, Error> {
        let matrix_b_t = Self::transpose(self.size, &self.matrix_b);

        // Parallel iteration over row indices.
        let result = (0..self.size)
            .into_par_iter()
            .map(|i| {
                let row_a = &self.matrix_a[i * self.size..(i + 1) * self.size];
                Self::compute_row(self.size, row_a, &matrix_b_t)
            })
            .reduce(Vec::new, Self::merge_matrix_chunks);

        Ok(result)
    }

    pub fn fork_join(&self, threshold: usize) -> Result<Vec<f64>, Error> {
        let matrix_b_t = Self::transpose(self.size, &self.matrix_b);
        let result = Self::fork_join_recursive(
            self.size,
            &self.matrix_a,
            &matrix_b_t,
            0,
            self.size,
            threshold,
        );

        Ok(result)
    }

    pub fn workers(&self, workers: usize) -> Result<Vec<f64>, Error> {
        let matrix_b_t = Self::transpose(self.size, &self.matrix_b);

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(|_| SystemError::CurrentExe(std::io::Error::last_os_error()))?;

        let result = pool.install(|| {
            (0..self.size)
                .into_par_iter()
                .map(|i| {
                    let row_a = &self.matrix_a[i * self.size..(i + 1) * self.size];
                    Self::compute_row(self.size, row_a, &matrix_b_t)
                })
                .reduce(Vec::new, Self::merge_matrix_chunks)
        });

        Ok(result)
    }
}

impl Reportable for MatrixMultiplication {
    fn name(&self) -> &'static str {
        "Matrices Multiplication"
    }
}

impl Measurable for MatrixMultiplication {}
