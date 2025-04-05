use crate::config::Config;

fn main() {
    let config = Config::default();

    logging::init(config.log_level, &logging::generate_log_name(&config.title))
        .unwrap_or_else(|err| {
            println!("Logger initialization failed. Error: {}", err);
            std::process::exit(1);
        });

    ui::start(config).unwrap_or_else(|err| {
        log::error!("{}", err);
        std::process::exit(1);
    });
}

mod config;
mod geometry {
    pub mod line2d;
    pub mod point2d;
}
mod graphics {
    pub mod grid;
    pub mod screen;
}
mod interpolation {
    pub mod examples;
    pub mod io;
    pub mod math;
    pub mod state;
    pub mod ui {
        pub mod input;
    }
}
mod logging;
mod state;
mod ui;
