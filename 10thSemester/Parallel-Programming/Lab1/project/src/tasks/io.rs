use crate::errors::{Error, SetupError, SystemError, TaskError, TeardownError};
use crate::task::benchmark::{BenchmarkMetadata, Benchmarkable};
use crate::task::executor::Executable;
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

pub struct WordCount {
    base_directory: PathBuf,
    files_amount: usize,
    words_per_file: usize,
}

impl WordCount {
    pub fn new(files_amount: usize, words_per_file: usize) -> Self {
        let mut base_directory = std::env::temp_dir();
        base_directory.push("MTTP-WordCount-Temp");

        Self {
            base_directory,
            files_amount,
            words_per_file,
        }
    }

    // Helper to recursively collect all file paths to process.
    // Sorting ensures deterministic chunking for parallel processes.
    fn collect_files(&self) -> Result<Vec<PathBuf>, Error> {
        let mut files = self.visit_dirs(&self.base_directory)?;
        files.sort();
        Ok(files)
    }

    fn visit_dirs(&self, directory: &Path) -> Result<Vec<PathBuf>, SystemError> {
        if !directory.is_dir() {
            Err(SystemError::DirectoryExpected(
                directory.display().to_string(),
            ))?;
        }

        let directory_entries =
            fs::read_dir(directory).map_err(SystemError::DirectoryRead)?;

        let mut paths = vec![];
        for entry in directory_entries {
            let entry = entry.map_err(SystemError::DirectoryEntry)?;
            let path = entry.path();
            if path.is_dir() {
                let inner_files = self.visit_dirs(&path)?;
                paths.extend(inner_files);
            } else {
                paths.push(path);
            }
        }

        Ok(paths)
    }

    // Core operation: Read a file from disk and count its words.
    fn count_words(path: &Path) -> Result<usize, Error> {
        let mut file = File::open(path).map_err(SystemError::FileOpen)?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(SystemError::FileRead)?;

        Ok(contents.split_whitespace().count())
    }
}

impl Benchmarkable for WordCount {}
impl Measurable for WordCount {}

impl Reportable for WordCount {
    fn name(&self) -> &'static str {
        "I/O-Bound: Word Count"
    }
}

impl Manageable for WordCount {
    fn setup(&mut self) -> Result<(), Error> {
        // Clean up any previous runs just in case.
        let _ = self.teardown();

        fs::create_dir_all(&self.base_directory)
            .map_err(SetupError::TemporaryDirectoryCreate)?;

        let text_content = "benchmark ".repeat(self.words_per_file);

        // Create deep directory structures to force recursive searching.
        for i in 0..self.files_amount {
            // Group every 100 files into a subdirectory.
            let sub_directory = self.base_directory.join(format!("folder_{}", i / 100));
            fs::create_dir_all(&sub_directory).map_err(SetupError::CreateFolder)?;

            let file_path = sub_directory.join(format!("file_{}.txt", i));
            let mut file = File::create(&file_path).map_err(SetupError::FileCreation)?;

            file.write_all(text_content.as_bytes())
                .map_err(SetupError::FileWrite)?;
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        // Remove the temporary directory and all generated files.
        fs::remove_dir_all(&self.base_directory)
            .map_err(TeardownError::TemporaryDirectoryDelete)?;
        Ok(())
    }
}

impl Executable for WordCount {
    fn run_sequential(&self) -> Result<(), Error> {
        let files = self.collect_files()?;
        let mut total_words = 0;

        for path in files {
            total_words += Self::count_words(&path)?;
        }

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Sequential,
            total_words
        );

        Ok(())
    }

    fn run_threads(&self, threads: usize) -> Result<(), Error> {
        if threads == 0 {
            return Ok(());
        }

        let files = self.collect_files()?;
        let chunk_size = files.len().div_ceil(threads);

        // Use an atomic to safely aggregate the total word count.
        let total_words = AtomicUsize::new(0);

        thread::scope(|s| {
            for chunk in files.chunks(chunk_size) {
                let shared_total = &total_words;
                s.spawn(move || {
                    let mut local_count = 0;
                    for path in chunk {
                        match Self::count_words(path) {
                            Ok(amount) => local_count += amount,
                            Err(err) => {
                                log::error!(
                                    "Error processing file {}: {}",
                                    path.display(),
                                    err
                                );
                                std::process::exit(1);
                            },
                        }
                    }
                    shared_total.fetch_add(local_count, Ordering::Relaxed);
                });
            }
        });

        log::info!(
            "TASK: {}, MODE: {}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Threads(threads),
            total_words.load(std::sync::atomic::Ordering::Relaxed)
        );

        Ok(())
    }

    fn run_workers(&self, workers: usize) -> Result<(), Error> {
        if workers == 0 {
            return Ok(());
        }

        let files = self.collect_files()?;

        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::RayonPoolBuild)?;

        let total_words: usize = pool.install(|| {
            use rayon::prelude::*;

            // Rayon dynamically handles the work stealing across I/O requests.
            files
                .into_par_iter()
                .map(|path| match Self::count_words(&path) {
                    Ok(amount) => amount,
                    Err(err) => {
                        log::error!("Error processing file {}: {}", path.display(), err);
                        0
                    },
                })
                .sum()
        });

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Workers(workers),
            total_words
        );

        Ok(())
    }

    fn run_process_chunk(
        &self, process_index: usize, total_processes: usize,
    ) -> Result<(), Error> {
        if total_processes == 0 {
            return Ok(());
        }

        let files = self.collect_files()?;

        let chunk_size = files.len().div_ceil(total_processes);
        let start_idx = process_index * chunk_size;
        let end_idx = std::cmp::min(files.len(), start_idx + chunk_size);

        let mut local_words = 0;

        if start_idx < end_idx {
            let chunk = files
                .get(start_idx..end_idx)
                .ok_or(TaskError::ChunkOutOfBounds(start_idx, end_idx))?;

            for path in chunk {
                match Self::count_words(path) {
                    Ok(count) => local_words += count,
                    Err(err) => {
                        log::error!("Error processing file {}: {}", path.display(), err);
                        std::process::exit(1);
                    },
                }
            }
        }

        // Print to stdout for the orchestrator to capture.
        println!("{}", local_words);

        Ok(())
    }

    fn aggregate_process_results(
        &self, total_processes: usize, results: Vec<String>,
    ) -> Result<(), Error> {
        let mut total_words = 0;

        for result_str in results {
            match result_str.trim().parse::<usize>() {
                Ok(count) => {
                    total_words += count;
                },
                Err(_) => Err(SystemError::FailedParsingChildOutput(result_str))?,
            }
        }

        log::info!(
            "TASK: {}, MODE: {:?}, RESULT: {}",
            self.name(),
            BenchmarkMetadata::Processes(total_processes),
            total_words,
        );

        Ok(())
    }
}
