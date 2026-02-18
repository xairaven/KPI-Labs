use crate::config::Config;
use crate::ui::app::App;
use eframe::NativeOptions;
use std::string::ToString;

pub const WINDOW_NAME: &str = "Wind Power";
pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 500.0;

pub fn start(config: Config) -> eframe::Result {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(WINDOW_NAME.to_string())
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_resizable(false)
            .with_icon(
                eframe::icon_data::from_png_bytes(&include_bytes!("../../assets/icon-64.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    eframe::run_native(
        WINDOW_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(App::new(cc, config)))),
    )
}
