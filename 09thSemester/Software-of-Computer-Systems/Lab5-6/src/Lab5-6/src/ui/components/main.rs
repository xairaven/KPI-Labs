use crate::context::Context;
use crate::errors::Error;
use crate::io::IoError;
use crate::ui::modals::error::ErrorModal;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct MainComponent {
    code: String,
    result: String,

    opened_file: Option<PathBuf>,
}

impl MainComponent {
    pub fn show(&mut self, context: &mut Context, ui: &mut egui::Ui) {
        if let Some(result) = context.ui.get_output() {
            self.result = result;
        }

        ui.horizontal(|ui| {
            ui.label("Code:");

            if ui
                .add(egui::TextEdit::singleline(&mut self.code).desired_width(500.0))
                .changed()
            {
                context.compiler.code = self.code.clone();
            };

            // Clear code field
            if ui.button("⟲").on_hover_text("Clear Code Field").clicked() {
                self.code = String::new();
                context.compiler.code = String::new();
            }

            // Open File
            if ui.button("📁").on_hover_text("Open File").clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("text", &["txt", "xai"])
                    .pick_file()
            {
                self.read_file(path, context);
            }

            if let Some(path) = &self.opened_file {
                // Reload file
                if ui.button("↺").on_hover_text("Reload File").clicked() {
                    self.read_file(path.clone(), context);
                }
                // Close file
                if ui.button("⊗").on_hover_text("Close File").clicked() {
                    self.opened_file = None;
                }
            }

            if !self.result.is_empty()
                && ui.button("🗐").on_hover_text("Copy Result").clicked()
            {
                ui.ctx().copy_text(self.result.trim().to_string());
            }
        });

        ui.separator();

        ui.centered_and_justified(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.result)
                        .interactive(false)
                        .code_editor(),
                );
            });
        });
    }

    fn read_file(&mut self, path: PathBuf, context: &mut Context) {
        match fs::read_to_string(&path) {
            Ok(text) => {
                self.code = text;
                context.compiler.code = self.code.clone();
                self.opened_file = Some(path.clone());
            },
            Err(error) => {
                let error: Error = IoError::ReadFile(error).into();
                ErrorModal::new(error).try_send_by(&context.ui.errors_tx);
            },
        }
    }
}
