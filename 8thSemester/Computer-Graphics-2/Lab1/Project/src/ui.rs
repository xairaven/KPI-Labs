use crate::config::Config;
use crate::ui::app::App;

pub const MIN_WINDOW_WIDTH: f32 = 900.0;
pub const MIN_WINDOW_HEIGHT: f32 = 550.0;

pub fn start(config: Config) -> eframe::Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("Computer Graphics 2: {}", config.title))
            .with_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_min_inner_size([MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT])
            .with_icon(
                eframe::icon_data::from_png_bytes(
                    &include_bytes!("../assets/icon-256.png")[..],
                )
                .unwrap_or_else(|err| {
                    log::error!("{}", format!("Failed to load app icon. {err}"));
                    std::process::exit(1);
                }),
            ),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        &config.title.clone(),
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
    )
}

pub(crate) mod app;
pub(crate) mod components {
    pub mod canvas;
    pub mod settings;
}
pub(crate) mod core;
pub(crate) mod modals;
pub(crate) mod styles {
    pub mod colors;
    pub mod strokes;
}
