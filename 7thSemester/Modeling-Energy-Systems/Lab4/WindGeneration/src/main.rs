use crate::config::Config;

fn main() {
    let app_config = Config::from_env().unwrap_or_else(|reason| {
        eprintln!("Error: {reason}");
        std::process::exit(1);
    });

    ui::core::start(app_config).unwrap_or_else(|reason| {
        eprintln!("{}", reason);
        std::process::exit(1);
    });
}

pub mod api_state;
pub mod config;
pub mod context;
pub mod errors {
    pub mod api;
    pub mod env;
    pub mod local_data;
}
pub mod model {
    pub mod wind_generator;
}
pub mod services {
    pub mod local_data;
    pub mod location;
    pub mod request;
    pub mod weather;
}
pub mod ui {
    pub mod app;
    pub mod components {
        pub mod weather_display;
        pub mod wind_calculator;
    }
    pub mod core;
    pub mod styles {
        pub mod colors;
    }
    pub mod windows {
        pub mod main;
    }
}
pub mod utils {
    pub mod date;
}
