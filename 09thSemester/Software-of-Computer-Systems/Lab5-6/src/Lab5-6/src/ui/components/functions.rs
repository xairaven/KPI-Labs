use crate::context::Context;
use egui::{DragValue, Grid};

#[derive(Default)]
pub struct FunctionsComponent;

impl FunctionsComponent {
    pub fn show(&self, context: &mut Context, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Run:");
        });

        ui.add_space(10.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Tokenizer").clicked() {
                context.ui.set_output(context.compiler.tokenize_report());
            }

            if ui.button("Syntax check").clicked() {
                context.ui.set_output(context.compiler.syntax_report());
            }

            if ui.button("Create Lexemes").clicked() {
                context.ui.set_output(context.compiler.lexer_report());
            }

            if ui.button("Build AST").clicked() {
                context.ui.set_output(context.compiler.ast_report());
            }

            if ui.button("Compute AST #1").clicked() {
                context.ui.set_output(context.compiler.compute_1_report());
            }

            if ui.button("Transform AST").clicked() {
                context.ui.set_output(context.compiler.transform_report());
            }

            if ui.button("Compute AST #2").clicked() {
                context.ui.set_output(context.compiler.compute_2_report());
            }

            if ui.button("Balance AST").clicked() {
                context.ui.set_output(context.compiler.balance_report());
            }

            if ui.button("Compute AST #3").clicked() {
                context.ui.set_output(context.compiler.compute_3_report());
            }

            if ui.button("Fold AST").clicked() {
                context.ui.set_output(context.compiler.folding_report());
            }

            if ui.button("Compute AST #4").clicked() {
                context.ui.set_output(context.compiler.compute_4_report());
            }

            ui.separator();

            if ui.button("Equivalent Forms").clicked() {
                context
                    .ui
                    .set_output(context.compiler.equivalent_forms_report());
            }
        });

        ui.separator();

        ui.vertical_centered_justified(|ui| {
            ui.add_space(10.0);

            ui.label("Vector System");

            ui.add_space(5.0);

            let processor_config = &mut context.compiler.system_configuration.processors;
            let time_config = &mut context.compiler.system_configuration.time;
            ui.group(|ui| {
                Grid::new("psc_config_grid")
                    .num_columns(3)
                    .spacing([40.0, 4.0])
                    .show(ui, |ui| {
                        ui.label("");
                        ui.label("PUs");
                        ui.label("Ticks");
                        ui.end_row();

                        ui.label("ADD: ");
                        Self::processor_drag(ui, &mut processor_config.add);
                        Self::time_drag(ui, &mut time_config.add);
                        ui.end_row();

                        ui.label("SUB: ");
                        Self::processor_drag(ui, &mut processor_config.sub);
                        Self::time_drag(ui, &mut time_config.sub);
                        ui.end_row();

                        ui.label("MUL: ");
                        Self::processor_drag(ui, &mut processor_config.mul);
                        Self::time_drag(ui, &mut time_config.mul);
                        ui.end_row();

                        ui.label("DIV: ");
                        Self::processor_drag(ui, &mut processor_config.div);
                        Self::time_drag(ui, &mut time_config.div);
                        ui.end_row();
                    });
            });

            ui.add_space(10.0);

            if ui.button("Simulate PCS").clicked() {
                context
                    .ui
                    .set_output(context.compiler.pcs_simulation_report());
            }

            if ui.button("PCS Config Reset").clicked() {
                context.compiler.system_configuration = Default::default();
            }

            ui.add_space(10.0);

            if ui.button("Optimization Research").clicked() {
                context
                    .ui
                    .set_output(context.compiler.optimization_research_report());
            }
        });
    }

    fn processor_drag(ui: &mut egui::Ui, value: &mut usize) {
        ui.add(DragValue::new(value).speed(1).range(0..=100));
    }

    fn time_drag(ui: &mut egui::Ui, value: &mut usize) {
        ui.add(DragValue::new(value).speed(1).range(0..=100).suffix(" t."));
    }
}
