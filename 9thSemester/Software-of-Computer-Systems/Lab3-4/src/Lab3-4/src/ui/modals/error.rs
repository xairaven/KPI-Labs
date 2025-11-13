use crate::context::Context;
use crate::errors::Error;
use crate::ui::modals::{Modal, ModalFields};
use egui::{RichText, Ui, WidgetText};

#[derive(Default)]
pub struct ErrorModal {
    modal_fields: ModalFields,
    message: WidgetText,
}

impl Modal for ErrorModal {
    fn show_content(&mut self, ui: &mut Ui, _ctx: &mut Context) {
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
    pub fn new(error: Error) -> Self {
        let message = format!("{}", error);

        Self {
            modal_fields: ModalFields::default()
                .with_title("❎ Error".to_string())
                .with_width(300.0),
            message: RichText::new(message).into(),
        }
    }

    pub fn try_send_by(self, tx: &crossbeam::channel::Sender<Self>) {
        if let Err(err) = tx.try_send(self) {
            log::error!("Failed to send modal: {err}");
        }
    }
}
