use crate::config::Config;
use crate::ui::app::App;

pub const DEFAULT_WINDOW_SETTINGS: WindowSettings = WindowSettings {
    min_width: 950.0,
    min_height: 550.0,
    project_title: "Lab",
};

pub struct WindowSettings {
    pub min_width: f32,
    pub min_height: f32,
    pub project_title: &'static str,
}

pub fn start(config: Config) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(crate::PROJECT_TITLE)
            .with_inner_size([
                DEFAULT_WINDOW_SETTINGS.min_width,
                DEFAULT_WINDOW_SETTINGS.min_height,
            ])
            .with_min_inner_size([
                DEFAULT_WINDOW_SETTINGS.min_width,
                DEFAULT_WINDOW_SETTINGS.min_height,
            ])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon-64.png")[..],
                )
                .unwrap_or_else(|err| {
                    log::error!("Failed to load app icon. {err}");
                    std::process::exit(1);
                }),
            ),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        crate::PROJECT_TITLE,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
    )
}

pub mod app;
pub mod context;
pub mod modals;
pub mod styles;

pub mod components {
    pub mod main;
    pub mod side;

    pub mod functions;
    pub mod settings;
}
