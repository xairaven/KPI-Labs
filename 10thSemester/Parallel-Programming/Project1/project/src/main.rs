// Hide console window on Windows in release mode
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;
use crate::logger::Logger;
use crate::ui::Ui;

const PROJECT_TITLE: &str = "MTPP Project #1 | Kovalov Oleksandr";

fn main() {
    let config = Config::from_file().unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });

    Logger::from_config(&config)
        .setup()
        .unwrap_or_else(|error| {
            Ui::native_panic_message(error);
            std::process::exit(1);
        });

    Ui::default().start(config).unwrap_or_else(|error| {
        Ui::native_panic_message(error);
        std::process::exit(1);
    });
}

mod backend;
mod config;
mod context;
mod errors;
mod graphics;
mod logger;
mod ui;
