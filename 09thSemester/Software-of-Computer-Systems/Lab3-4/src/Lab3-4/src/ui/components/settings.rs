use crate::context::Context;

#[derive(Default)]
pub struct SettingsComponent;

impl SettingsComponent {
    pub fn show(&self, context: &mut Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Settings");
        });

        ui.add_space(10.0);

        ui.checkbox(&mut context.compiler.pretty_output, "Pretty Output");

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Save Config").clicked() {
                context.save_config();
            }
        });
    }
}
