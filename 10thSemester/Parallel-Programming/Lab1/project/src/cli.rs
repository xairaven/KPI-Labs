use clap::Parser;
use log::LevelFilter;

#[derive(Parser)]
pub struct InputArgs {
    #[arg(
        short,
        long,
        help = "Task number to execute (1-5)",
        long_help = "Specify the task number to execute.
    Valid options are strictly from 1 to 5 inclusive. Any other input will be rejected by the parser.

    Tasks:
    1) CPU-bound operation calculating Pi using the Monte Carlo method.
    2) CPU-bound operation for factoring large numbers.
    3) CPU-bound operation finding prime numbers within a specific range.
    4) Memory-bound operation that transposes a 10000x10000 matrix.
    5) I/O-bound operation involving recursive word counting in randomly generated text files."
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

    // Enable `processes mode` for distributed execution.
    // Not for user -- program will use it internally when needed.
    #[arg(short, long, hide = true)]
    pub processes_mode: bool,

    // Identifies the specific chunk of work this child process should handle.
    // E.g., process 0, process 1, process 2.
    // Not for user -- program will use it internally when needed.
    #[arg(long, hide = true)]
    pub process_index: Option<usize>,

    // Tells the child process how many chunks the data is divided into.
    // Not for user -- program will use it internally when needed.
    #[arg(long, hide = true)]
    pub total_processes: Option<usize>,
}
