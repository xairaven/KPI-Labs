use crate::context::Context;

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
    }
}
