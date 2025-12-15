use crate::context::Context;
use crate::ui::components::functions::FunctionsComponent;
use crate::ui::components::settings::SettingsComponent;

#[derive(Debug, Default)]
pub struct SideComponent;

impl SideComponent {
    pub fn show(&self, context: &mut Context, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            FunctionsComponent.show(context, ui);

            ui.add_space(10.0);

            ui.separator();

            ui.add_space(10.0);

            SettingsComponent.show(context, ui);
        });
    }
}
