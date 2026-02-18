use crate::compiler::ast::tree::AbstractSyntaxTree;
use crate::compiler::pcs::SystemConfiguration;
use crate::compiler::pcs::vector::{SimulationResult, VectorSystemSimulator};
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;

pub struct Researcher<'a> {
    forms: &'a Vec<AbstractSyntaxTree>,
    configuration: &'a SystemConfiguration,
}

pub struct OptimizationReport {
    pub index: usize,
    pub canonical_string: String,
    pub result: SimulationResult,
}

impl<'a> Researcher<'a> {
    pub fn new(
        equivalent_forms: &'a Vec<AbstractSyntaxTree>,
        system_configuration: &'a SystemConfiguration,
    ) -> Self {
        Self {
            forms: equivalent_forms,
            configuration: system_configuration,
        }
    }

    pub fn run(&self) -> Result<Vec<OptimizationReport>, String> {
        let mut results = Vec::new();

        for (index, form) in self.forms.iter().enumerate() {
            let simulator = VectorSystemSimulator::new(form, self.configuration);
            let result = simulator.simulate();

            results.push(OptimizationReport {
                index,
                canonical_string: form.to_canonical_string(),
                result,
            });
        }

        Ok(results)
    }
}

impl Reporter {
    pub fn generate_optimization_report(reports: &[OptimizationReport]) -> String {
        let mut buffer = StringBuffer::default();

        buffer.add_line("Optimization Research".to_string());
        buffer.add_line(
            "Goal: Find the optimal parallel form for the given architecture."
                .to_string(),
        );
        buffer.add_line("-".repeat(100));

        // First line contains system configuration
        match reports.first() {
            Some(first_report) => {
                let configuration = &first_report.result.configuration;
                let processors = &configuration.processors;
                let time = &configuration.time;

                buffer.add_line(format!(
                    "System Config: Add({}), Sub({}), Mul({}), Div({}) | Costs: A={}, S={}, M={}, D={}",
                    processors.add,
                    processors.sub,
                    processors.mul,
                    processors.div,
                    time.add,
                    time.sub,
                    time.mul,
                    time.div,
                ));

                buffer.add_line("-".repeat(100));
            },
            None => {
                buffer.add_line("No optimization results available.".to_string());
                return buffer.get();
            },
        }

        // Table header
        buffer.add_line(format!(
            "{:<4} | {:<40} | {:<5} | {:<5} | {:<8} | {:<8}",
            "ID", "Form (Snippet)", "T1", "Tp", "Kp (Spd)", "Ep (Eff)"
        ));
        buffer.add_line("-".repeat(100));

        let mut best_tp = usize::MAX;
        let mut best_efficiency = f64::MAX;
        let mut best_index = 0;

        // Table rows
        for report in reports {
            let result = &report.result;
            let form_str = &report.canonical_string;

            // Shortening variable names for clarity
            let short_form = if form_str.len() > 37 {
                format!("{}...", &form_str[0..37])
            } else {
                form_str.clone()
            };

            buffer.add_line(format!(
                "{:<4} | {:<40} | {:<5} | {:<5} | {:<8.4} | {:<8.4}",
                report.index,
                short_form,
                result.t1,
                result.tp,
                result.speedup,
                result.efficiency
            ));

            // Searching for optimal form
            if result.tp < best_tp {
                best_tp = result.tp;
                best_index = report.index;
            } else if result.tp == best_tp && result.efficiency > best_efficiency {
                best_tp = result.tp;
                best_index = report.index;
                best_efficiency = result.efficiency;
            }
        }

        buffer.add_line("-".repeat(100));

        // Conclusion with the best form
        let best_report = &reports[best_index];
        let best_result = &best_report.result;
        buffer.add_line(format!("\nOptimal Form Found: ID #{}", best_index));
        buffer.add_line(format!("Expression: {}", best_report.canonical_string));
        buffer.add_line(format!(
            "Metrics: T1 = {}, Tp = {} ticks, Speedup = {:.4}, Efficiency = {:.4}",
            best_result.t1, best_result.tp, best_result.speedup, best_result.efficiency
        ));

        buffer.get()
    }
}
