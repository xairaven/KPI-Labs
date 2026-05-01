use crate::context::Context;
use crate::ui::components::canvas::CanvasComponent;
use crate::ui::components::settings::SettingsComponent;

pub struct Workspace {
    pub canvas: CanvasComponent,
    pub settings: SettingsComponent,
}

impl Workspace {
    pub fn new(_context: &Context) -> Self {
        Self {
            canvas: CanvasComponent,
            settings: SettingsComponent::default(),
        }
    }

    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        self.settings.show(ui, context);
        self.canvas.show(ui, context);
    }
}
