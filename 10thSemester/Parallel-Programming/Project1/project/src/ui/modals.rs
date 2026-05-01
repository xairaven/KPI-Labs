use crate::context::Context;
use crate::ui::modals::error::ErrorModal;

#[derive(Debug, Default)]
pub struct ModalsHandler {
    errors: Vec<ErrorModal>,
}

impl ModalsHandler {
    pub fn handle_errors(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        // Getting modals from the channels (in context).
        if let Ok(modal) = ctx.error_modals_rx.try_recv() {
            self.errors.push(modal);
        }

        // Showing modals.
        self.show_opened_modals(ui, ctx);
    }

    fn show_opened_modals(&mut self, ui: &mut egui::Ui, ctx: &Context) {
        let mut closed_modals: Vec<usize> = vec![];

        for (index, modal) in self.errors.iter_mut().enumerate() {
            modal.show(ui, ctx);

            if modal.is_closed() {
                closed_modals.push(index);
            }
        }

        closed_modals.iter().for_each(|index| {
            self.errors.remove(*index);
        });
    }
}

#[derive(Debug, Clone)]
pub struct ModalFields {
    pub id: egui::Id,
    pub title: String,
    pub width: f32,

    pub is_open: bool,
}

impl Default for ModalFields {
    fn default() -> Self {
        Self {
            id: egui::Id::new(rand::random::<i64>()),
            title: "Modal".to_string(),
            is_open: true,
            width: 100.0,
        }
    }
}

impl ModalFields {
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

pub trait Modal: Send + Sync {
    fn show(&mut self, ui: &egui::Ui, ctx: &Context) {
        if !self.is_closed() {
            let modal = egui::Modal::new(self.modal_fields().id).show(ui.ctx(), |ui| {
                ui.set_width(self.modal_fields().width);

                ui.vertical_centered_justified(|ui| {
                    ui.heading(&self.modal_fields().title);
                });

                ui.add_space(16.0);

                self.show_content(ui, ctx);
            });

            if modal.should_close() {
                self.close()
            }
        }
    }

    fn is_closed(&self) -> bool {
        !self.modal_fields().is_open
    }

    fn show_content(&mut self, ui: &mut egui::Ui, ctx: &Context);
    fn close(&mut self);
    fn modal_fields(&self) -> &ModalFields;
}

pub mod error;
