use crate::cli::Cli;
use colored::Colorize;

fn main() {
    let run_result = Cli::run();

    if let Err(e) = run_result {
        eprintln!("{}. {e}", "Error".red().bold());
    }
}

pub mod cli;
pub mod compiler;
pub mod error;
pub mod io;
pub mod utils;
