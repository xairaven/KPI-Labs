// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;
use crate::logs::Logger;

fn main() {
    let config = Config::from_file().unwrap_or_else(|err| {
        eprintln!("Error. {err}");
        std::process::exit(1);
    });

    Logger::default()
        .with_file_title(&config.project_title)
        .with_format(&config.log_format)
        .with_level(config.log_level)
        .setup()
        .unwrap_or_else(|err| {
            eprintln!("Error. {err}");
            std::process::exit(1);
        });

    log::info!("Starting application.");
    log::info!("Config loaded: {config:#?}");
    log::info!("Logger initialized.");

    ui::start(config).unwrap_or_else(|err| {
        log::error!("{err}");
        std::process::exit(1);
    });
}

pub mod compiler;
pub mod config;
pub mod context;
pub mod errors;
pub mod io;
pub mod logs;
pub mod ui;
pub mod utils;
