use crate::cli::InputArgs;
use crate::errors::{CliError, Error};
use crate::logs::Logger;
use crate::task::tasks;
use clap::Parser;

fn main() {
    let args = InputArgs::parse();

    Logger::from_args(&args).setup().unwrap_or_else(|error| {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    });

    match run_task(args) {
        Ok(report) => println!("{}", report),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        },
    }
}

fn run_task(args: InputArgs) -> Result<String, Error> {
    let task_index = args.task;
    let tasks = tasks();

    if !(1..=tasks.len()).contains(&task_index) {
        Err(CliError::InvalidTaskNumber(task_index))?;
    }

    let mut task = tasks
        .into_iter()
        .nth(task_index - 1)
        .ok_or(CliError::InvalidTaskNumber(task_index))?;

    // Process child
    if args.processes_mode {
        let process_index = args.process_index.ok_or(CliError::MissingProcessIndex)?;
        let total_processes = args
            .total_processes
            .ok_or(CliError::MissingTotalProcesses)?;
        task.run_process_chunk(process_index, total_processes)?;
        std::process::exit(0);
    }

    let report = task.report(task_index)?;

    Ok(report.text())
}

mod cli;
mod errors;
mod logs;
mod task;

mod tasks {
    pub mod factorization;
    pub mod io;
    pub mod monte_carlo;
    pub mod primes;
    pub mod transpose;
}
