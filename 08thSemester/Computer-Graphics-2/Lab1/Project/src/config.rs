use egui::ThemePreference;
use log::LevelFilter;

pub const WINDOW_TITLE: &str = "Lab 1";

pub struct Config {
    pub title: String,
    pub log_level: LevelFilter,
    pub theme: ThemePreference,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            title: WINDOW_TITLE.to_string(),
            log_level: LevelFilter::Off,
            theme: ThemePreference::Dark,
        }
    }
}
