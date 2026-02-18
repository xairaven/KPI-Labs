use crate::ui::app::App;
use egui::{CentralPanel, SidePanel};

pub fn show(app: &mut App, ui: &mut egui::Ui, _ctx: &egui::Context) {
    SidePanel::right("CANVAS_PANEL")
        .resizable(false)
        .default_width(app.settings.width)
        .show_separator_line(true)
        .show_inside(ui, |ui| {
            app.settings.show(&mut app.state, &mut app.canvas, ui);
        });

    CentralPanel::default().show_inside(ui, |ui| {
        app.canvas.show(&mut app.state, ui);
    });
}
