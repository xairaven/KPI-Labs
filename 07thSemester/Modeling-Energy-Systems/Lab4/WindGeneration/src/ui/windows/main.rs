use crate::config::Config;
use crate::context::Context;
use crate::ui::components;
use crate::ui::core::WINDOW_WIDTH;
use egui::{CentralPanel, SidePanel};

pub fn show(config: &Config, context: &mut Context, ui: &mut egui::Ui) {
    if context.api_state.should_update() {
        let _ = context.api_state.update(
            &config.api_key,
            &context.local_data[context.selected_city_index].name,
        );
    }

    SidePanel::left("WindCalculator")
        .resizable(false)
        .exact_width(WINDOW_WIDTH / 2.0)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            components::weather_display::show(config, context, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        components::wind_calculator::show(context, ui);
    });

    ui.ctx().request_repaint();
}
