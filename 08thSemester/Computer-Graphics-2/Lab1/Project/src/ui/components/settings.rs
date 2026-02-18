use crate::approximation;
use crate::approximation::examples::Functions;
use crate::approximation::ui::input::InputModal;
use crate::state::State;
use crate::ui::components::canvas::Canvas;
use crate::ui::modals::message::MessageModal;
use crate::ui::styles::colors;
use egui::{DragValue, Grid, RichText};

pub struct Settings {
    pub width: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self { width: 250.0 }
    }
}

impl Settings {
    pub fn show(&mut self, state: &mut State, canvas: &mut Canvas, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Settings");
            });

            ui.add_space(10.0);

            ui.horizontal(|ui| {
                ui.label("Pixels per Centimeter:");
                ui.add(
                    DragValue::new(&mut canvas.screen_params.px_per_cm)
                        .speed(1)
                        .range(5.0..=100.0)
                        .suffix(" cm."),
                );
            });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Offset").clicked() {
                    canvas.screen_params.offset = Default::default();
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset to Default Settings").clicked() {
                    self.reset_to_defaults(state, canvas);
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            Grid::new("Approximation.Settings")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Status: ");
                    if state.approximation_state.is_initialized() {
                        ui.label(RichText::new("Initialized").color(colors::LIME));
                    } else {
                        ui.label(RichText::new("Uninitialized").color(colors::RED));
                    }
                    ui.end_row();

                    if state.approximation_state.is_initialized() {
                        ui.checkbox(&mut state.is_curve_enabled, "Enable Curve");
                        ui.end_row();
                        ui.checkbox(&mut state.is_line_enabled, "Enable Line");
                        ui.end_row();
                        ui.checkbox(&mut state.is_parabola_enabled, "Enable Parabola");
                        ui.end_row();

                        ui.label("Curve Color: ");
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut state.curve_color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        ui.end_row();

                        ui.label("Linear Color: ");
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut state.linear_color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        ui.end_row();

                        ui.label("Quadratic Color: ");
                        egui::color_picker::color_edit_button_srgba(
                            ui,
                            &mut state.quadratic_color,
                            egui::color_picker::Alpha::Opaque,
                        );
                        ui.end_row();
                    }
                });

            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                if ui.button("Input Points").clicked() {
                    let modal = Box::new(InputModal::default());
                    if state.modals_tx.try_send(modal).is_err() {
                        log::error!("Failed to send modal");
                    }
                }
            });

            ui.vertical_centered_justified(|ui| {
                if ui.button("Reset Initialization").clicked() {
                    state.approximation_state = Default::default();
                }
            });

            ui.add_space(10.0);
            ui.vertical_centered_justified(|ui| {
                if ui.button("Load From...").clicked() {
                    if let Err(err) = approximation::io::load_with_file_pick(
                        &mut state.approximation_state,
                    ) {
                        let modal = Box::new(MessageModal::error(&err.to_string()));
                        let _ = state.modals_tx.try_send(modal);
                    }
                }
                if state.approximation_state.is_initialized()
                    && ui.button("Save To...").clicked()
                {
                    if let Err(err) = approximation::io::save_with_file_pick(
                        &mut state.approximation_state,
                    ) {
                        let modal = Box::new(MessageModal::error(&err.to_string()));
                        let _ = state.modals_tx.try_send(modal);
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.vertical_centered_justified(|ui| {
                ui.label("Examples:");
                for function in Functions::vec() {
                    if ui.button(function.name()).clicked() {
                        state.approximation_state.points_view = function.points();
                        if let Err(err) = state.approximation_state.initialize() {
                            state.approximation_state = Default::default();
                            let modal = Box::new(MessageModal::error(&err.to_string()));
                            let _ = state.modals_tx.try_send(modal);
                        }
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.add_space(10.0);

            ui.collapsing("Grid Settings", |ui| {
                ui.checkbox(&mut state.grid.is_enabled, "Enable Grid");
                ui.checkbox(
                    &mut canvas.screen_params.is_dragging_offset_enabled,
                    "Enable Drag & Offset",
                );

                ui.add_space(5.0);

                ui.horizontal(|ui| {
                    ui.label("Unit Length:");
                    ui.add(
                        DragValue::new(&mut canvas.screen_params.unit_length)
                            .speed(0.1)
                            .range(0.1..=f32::MAX)
                            .suffix(" cm."),
                    );
                });

                ui.add_space(10.0);

                ui.label(RichText::new("Colors:").strong());
                Grid::new("GridColors").num_columns(2).show(ui, |ui| {
                    ui.label("Axis X:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut state.grid.axis_x_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Axis Y:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut state.grid.axis_y_color,
                        egui::color_picker::Alpha::Opaque,
                    );

                    ui.end_row();

                    ui.label("Grid:");
                    egui::color_picker::color_edit_button_srgba(
                        ui,
                        &mut state.grid.grid_color,
                        egui::color_picker::Alpha::Opaque,
                    );
                });

                ui.add_space(10.0);

                ui.vertical_centered(|ui| {
                    if ui.button("Reset Settings").clicked() {
                        state.grid = Default::default();
                        canvas.screen_params.is_dragging_offset_enabled = true;
                        canvas.screen_params.offset = (0.0, 0.0);
                        canvas.screen_params.unit_length = 1.0;
                    }
                });
            });
        });
    }

    fn reset_to_defaults(&mut self, state: &mut State, canvas: &mut Canvas) {
        canvas.screen_params = Default::default();
        *state = Default::default();
    }
}
