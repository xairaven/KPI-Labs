use clap::Parser;
use log::LevelFilter;
use thiserror::Error;

#[derive(Parser)]
pub struct InputArgs {
    #[arg(
        short,
        long,
        help = "Task number to execute (1-4)",
        long_help = "Specify the task number to execute.
    Valid options are strictly from 1 to 4 inclusive. Any other input will be rejected by the parser.

    Tasks:
    1) Count HTML Tags
    2) Min-Max-Mean-Avg of the big array.
    3) Matrices Multiplication.
    4) Video-Filter"
    )]
    pub task: usize,

    #[arg(
        short,
        long,
        value_enum,
        default_value = "off",
        help = "Set the logging level (off, error, warn, info, debug, trace)"
    )]
    pub log_level: LevelFilter,
}

#[derive(Debug, Error)]
pub enum CliError {
    #[error("Invalid task number: {0}. Valid options are 1-4.")]
    UnknownTask(usize),
}
