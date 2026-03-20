use crate::errors::{Error, SystemError};
use crate::task::benchmark::{BenchmarkKind, Benchmarkable};
use crate::task::executable::{Executable, RunMode};
use crate::task::lifecycle::Manageable;
use crate::task::measure::Measurable;
use crate::task::report::Reportable;
use crate::tasks::TaskLogicError;
use rand::RngExt;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use std::collections::HashMap;

#[derive(Debug)]
pub struct HtmlTags {
    pub documents_amount: usize,

    html_documents: Vec<String>,
}

// Builder
impl HtmlTags {
    pub fn with_documents_amount(amount: usize) -> Self {
        Self {
            documents_amount: amount,
            html_documents: vec![],
        }
    }
}

impl HtmlTags {
    // Pure function to count tags in a single document.
    // Represents the "Map" phase of our patterns.
    fn count_tags_in_doc(document: &str) -> HashMap<String, usize> {
        let mut counts = HashMap::new();

        // Very basic HTML tag extraction looking for text between '<' and '>'.
        // We split by '<', take the text up to '>', and grab the first word.
        for part in document.split('<').skip(1) {
            if let Some(tag_content) = part.split('>').next() {
                let tag_name = tag_content.split_whitespace().next().unwrap_or("");
                let tag_name = tag_name.replace("/", "").to_lowercase();

                if !tag_name.is_empty() {
                    *counts.entry(tag_name).or_insert(0) += 1;
                }
            }
        }

        counts
    }

    // Pure function to merge two HashMaps together.
    // Represents the "Reduce" or "Join" phase of our patterns.
    fn merge_counts(
        mut a: HashMap<String, usize>, b: HashMap<String, usize>,
    ) -> HashMap<String, usize> {
        for (key, value) in b {
            *a.entry(key).or_insert(0) += value;
        }
        a
    }

    // Recursive Fork-Join implementation.
    fn fork_join_recursive(docs: &[String], threshold: usize) -> HashMap<String, usize> {
        // Base case: if the chunk is small enough, process it sequentially.
        if docs.len() <= threshold {
            let mut local_counts = HashMap::new();
            for doc in docs {
                let doc_counts = Self::count_tags_in_doc(doc);
                local_counts = Self::merge_counts(local_counts, doc_counts);
            }
            return local_counts;
        }

        // Recursive case: split the workload in half.
        let mid = docs.len() / 2;
        let (left_docs, right_docs) = docs.split_at(mid);

        // Rayon's join guarantees these two closures can run on different threads.
        let (left_result, right_result) = rayon::join(
            || Self::fork_join_recursive(left_docs, threshold),
            || Self::fork_join_recursive(right_docs, threshold),
        );

        Self::merge_counts(left_result, right_result)
    }
}

impl Benchmarkable for HtmlTags {
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

impl Manageable for HtmlTags {
    fn setup(&mut self) -> Result<(), Error> {
        let available_tags =
            ["html", "head", "body", "div", "span", "p", "a", "h1", "img"];
        let mut rng = rand::rng();

        // Generate HTML documents with random tags.
        for _ in 0..self.documents_amount {
            let mut document = String::new();
            let tags_in_document = rng.random_range(50..500);

            for _ in 0..tags_in_document {
                let tag_index = rng.random_range(0..available_tags.len());
                let tag = available_tags
                    .get(tag_index)
                    .ok_or(TaskLogicError::IndexOutOfBounds(tag_index))?;
                document.push_str(&format!("<{}>random text</{}> ", tag, tag));
            }

            self.html_documents.push(document);
        }

        Ok(())
    }

    fn teardown(&mut self) -> Result<(), Error> {
        self.html_documents.clear();
        Ok(())
    }
}

impl Executable for HtmlTags {
    fn supported_modes(&self) -> Vec<RunMode> {
        vec![
            RunMode::Sequential,
            RunMode::MapReduce,
            RunMode::ForkJoin,
            RunMode::WorkerPool,
        ]
    }

    fn run(&self, kind: BenchmarkKind) -> Result<(), Error> {
        let unique_tags_found = match kind {
            BenchmarkKind::Sequential => self.sequential(),
            BenchmarkKind::MapReduce { .. } => self.map_reduce(),
            BenchmarkKind::ForkJoin { threshold } => self.fork_join(threshold),
            BenchmarkKind::WorkerPool { workers } => self.workers(workers),
            _ => return Ok(()), // Supported methods check guarantees this won't happen.
        }?;

        log::info!(
            "TASK: {}, MODE: {}, UNIQUE TAGS PROCESSED: {}",
            self.name(),
            kind,
            unique_tags_found
        );

        Ok(())
    }
}

// Modes
impl HtmlTags {
    pub fn sequential(&self) -> Result<usize, Error> {
        let map = self
            .html_documents
            .iter()
            .map(|document| Self::count_tags_in_doc(document.as_str()))
            .fold(HashMap::new(), Self::merge_counts);

        Ok(map.len())
    }

    pub fn map_reduce(&self) -> Result<usize, Error> {
        let map = self
            .html_documents
            .par_iter()
            .map(|doc| Self::count_tags_in_doc(doc))
            .reduce(HashMap::new, Self::merge_counts);

        Ok(map.len())
    }

    pub fn fork_join(&self, threshold: usize) -> Result<usize, Error> {
        let map = Self::fork_join_recursive(&self.html_documents, threshold);

        Ok(map.len())
    }

    pub fn workers(&self, workers: usize) -> Result<usize, Error> {
        // Creating a custom thread pool isolated for this specific execution.
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(SystemError::WorkerPoolBuild)?;

        // Executing the parallel iteration inside our custom pool.
        let map = pool.install(|| {
            self.html_documents
                .par_iter()
                .map(|doc| Self::count_tags_in_doc(doc))
                .reduce(HashMap::new, Self::merge_counts)
        });

        Ok(map.len())
    }
}

impl Reportable for HtmlTags {
    fn name(&self) -> &'static str {
        "HTML Tag Counting"
    }
}

impl Measurable for HtmlTags {}
