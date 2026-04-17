use crate::cli::InputArgs;
use crate::logs::Logger;
use crate::tasks::BenchmarkRunner;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    Logger::from_args(&args).setup().unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    });

    match BenchmarkRunner::default().run(args) {
        Ok(report) => println!("{}", report),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        },
    }
}

mod cli;
mod errors;
mod logs;
mod tasks;

mod task {
    pub mod benchmark;
    pub mod executable;
    pub mod lifecycle;
    pub mod measure;
    pub mod report;
}
