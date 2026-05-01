use crate::context::Context;
use crate::errors::ProjectError;
use crate::ui::modals::{Modal, ModalFields};
use egui::{RichText, WidgetText};

#[derive(Debug, Default, Clone)]
pub struct ErrorModal {
    modal_fields: ModalFields,
    message: WidgetText,
}

impl Modal for ErrorModal {
    fn show_content(&mut self, ui: &mut egui::Ui, _ctx: &Context) {
        ui.label(self.message.clone());

        ui.add_space(16.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Close").clicked() {
                self.close()
            }
        });
    }

    fn close(&mut self) {
        self.modal_fields.is_open = false;
    }

    fn modal_fields(&self) -> &ModalFields {
        &self.modal_fields
    }
}

impl ErrorModal {
    pub fn new(error: ProjectError) -> Self {
        Self {
            modal_fields: ModalFields::default()
                .with_title("❎ Error".to_string())
                .with_width(300.0),
            message: RichText::new(error.to_string()).into(),
        }
    }
}
