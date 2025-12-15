use crate::compiler::ast::tree::{
    AbstractSyntaxTree, AstNode, BinaryOperationKind, UnaryOperationKind,
};
use crate::compiler::pcs::{SystemConfiguration, TimeConfiguration};
use crate::compiler::reports::Reporter;
use crate::utils::StringBuffer;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperationType {
    Add,
    Sub,
    Mul,
    Div,
    Load, // Represents immediate operations or variable loads (0 latency)
}

impl OperationType {
    /// Returns the latency (execution time in ticks) for this operation type.
    fn latency(&self, time_config: &TimeConfiguration) -> usize {
        match self {
            Self::Add => time_config.add,
            Self::Sub => time_config.sub,
            Self::Mul => time_config.mul,
            Self::Div => time_config.div,
            Self::Load => 0,
        }
    }

    fn from_binary(kind: &BinaryOperationKind) -> Self {
        match kind {
            BinaryOperationKind::Plus => Self::Add,
            BinaryOperationKind::Minus => Self::Sub,
            BinaryOperationKind::Multiply => Self::Mul,
            BinaryOperationKind::Divide => Self::Div,
            _ => OperationType::Load,
        }
    }
}

impl std::fmt::Display for OperationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Self::Add => "ADD",
            Self::Sub => "SUB",
            Self::Mul => "MUL",
            Self::Div => "DIV",
            Self::Load => "LOAD",
        };
        write!(f, "{}", str)
    }
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    operation_type: OperationType,
    dependencies: Vec<usize>,
    // String representation of the specific operation (e.g., "a + b")
    display_name: String,
    // Rank or depth in the AST, used for priority heuristic
    rank: usize,
}

#[derive(Debug, Clone)]
pub struct ScheduledTask {
    pub task_id: usize,
    pub name: String,
    pub start_time: usize,
    pub end_time: usize,
    pub processor_type: OperationType,
    // The specific index of the processor unit (e.g., ADD #0, ADD #1)
    pub processor_index: usize,
}

#[derive(Debug, Clone)]
pub struct TickLog {
    pub tick: usize,
    // Maps "ProcessorName" (e.g., "ADD #1") -> Status String (e.g., "[Stage 2: x+y]")
    pub pipelines_state: HashMap<String, String>,
    pub ready_queue: Vec<String>,
}

pub struct SimulationResult {
    pub schedule: Vec<ScheduledTask>,
    pub tick_logs: Vec<TickLog>,
    pub t1: usize,
    pub tp: usize,
    pub speedup: f64,
    pub efficiency: f64,

    pub configuration: SystemConfiguration,
}

pub struct VectorSystemSimulator<'a> {
    ast: &'a AbstractSyntaxTree,
    configuration: &'a SystemConfiguration,
}

impl<'a> VectorSystemSimulator<'a> {
    pub fn new(
        ast: &'a AbstractSyntaxTree, configuration: &'a SystemConfiguration,
    ) -> Self {
        Self { ast, configuration }
    }

    pub fn simulate(&self) -> SimulationResult {
        // 1. Convert AST to a flat Task Graph
        let tasks = Self::flatten_ast(self.ast);

        // 2. Run Pipelined List Scheduling
        let (schedule, tick_logs) = self.run_pipelined_scheduling(tasks);

        // 3. Calculate metrics
        // Tp = Finish time of the last task
        let tp = schedule.iter().map(|t| t.end_time).max().unwrap_or(0);

        // T1 = Sum of execution times of all computational tasks (excluding Loads)
        // We use pure latency for T1 calculation
        let t1: usize = schedule
            .iter()
            .filter(|t| t.processor_type != OperationType::Load)
            .map(|t| t.end_time - t.start_time)
            .sum();

        let total_processors = self.configuration.processors.total();

        let speedup = if tp > 0 { t1 as f64 / tp as f64 } else { 0.0 };
        let efficiency = if total_processors > 0 {
            speedup / total_processors as f64
        } else {
            0.0
        };

        SimulationResult {
            schedule,
            tick_logs,
            t1,
            tp,
            speedup,
            efficiency,
            configuration: self.configuration.clone(),
        }
    }

    /// Recursively traverses the AST to build tasks.
    fn flatten_ast(ast: &AbstractSyntaxTree) -> HashMap<usize, Task> {
        let mut tasks = HashMap::new();
        let mut id_counter = 0;
        Self::traverse_node(&ast.peek, &mut tasks, &mut id_counter);
        tasks
    }

    /// Helper: returns (node_id, rank, text_representation)
    fn traverse_node(
        node: &AstNode, tasks: &mut HashMap<usize, Task>, counter: &mut usize,
    ) -> (usize, usize, String) {
        let current_id = *counter;
        *counter += 1;

        match node {
            AstNode::BinaryOperation {
                operation,
                left,
                right,
            } => {
                let (left_id, left_rank, left_text) =
                    Self::traverse_node(left, tasks, counter);
                let (right_id, right_rank, right_text) =
                    Self::traverse_node(right, tasks, counter);

                // Rank is max depth of children + 1
                let rank = std::cmp::max(left_rank, right_rank) + 1;

                let op_symbol = match operation {
                    BinaryOperationKind::Plus => "+",
                    BinaryOperationKind::Minus => "-",
                    BinaryOperationKind::Multiply => "*",
                    BinaryOperationKind::Divide => "/",
                    _ => "?",
                };
                let display_name =
                    format!("({} {} {})", left_text, op_symbol, right_text);

                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::from_binary(operation),
                        dependencies: vec![left_id, right_id],
                        display_name: display_name.clone(),
                        rank,
                    },
                );
                (current_id, rank, display_name)
            },
            AstNode::UnaryOperation {
                operation,
                expression,
            } => {
                let (child_id, child_rank, child_text) =
                    Self::traverse_node(expression, tasks, counter);

                let rank = child_rank + 1;
                let op_type = match operation {
                    UnaryOperationKind::Minus => OperationType::Sub,
                    _ => OperationType::Load,
                };
                let op_symbol = match operation {
                    UnaryOperationKind::Minus => "-",
                    UnaryOperationKind::Not => "!",
                };
                let display_name = format!("{}{}", op_symbol, child_text);

                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: op_type,
                        dependencies: vec![child_id],
                        display_name: display_name.clone(),
                        rank,
                    },
                );
                (current_id, rank, display_name)
            },
            AstNode::Number(n) => {
                let text = format!("{:.1}", n);
                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::Load,
                        dependencies: vec![],
                        display_name: text.clone(),
                        rank: 0,
                    },
                );
                (current_id, 0, text)
            },
            AstNode::Identifier(s) => {
                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::Load,
                        dependencies: vec![],
                        display_name: s.clone(),
                        rank: 0,
                    },
                );
                (current_id, 0, s.clone())
            },
            AstNode::FunctionCall { name, .. } => {
                let text = format!("{}()", name);
                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::Load,
                        dependencies: vec![],
                        display_name: text.clone(),
                        rank: 0,
                    },
                );
                (current_id, 0, text)
            },
            AstNode::ArrayAccess { identifier, .. } => {
                let text = format!("{}[..]", identifier);
                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::Load,
                        dependencies: vec![],
                        display_name: text.clone(),
                        rank: 0,
                    },
                );
                (current_id, 0, text)
            },
            _ => {
                tasks.insert(
                    current_id,
                    Task {
                        id: current_id,
                        operation_type: OperationType::Load,
                        dependencies: vec![],
                        display_name: "?".to_string(),
                        rank: 0,
                    },
                );
                (current_id, 0, "?".to_string())
            },
        }
    }

    /// Runs the simulation assuming pipelined processors.
    /// A processor unit can accept a new input 1 tick after the previous start.
    fn run_pipelined_scheduling(
        &self, tasks: HashMap<usize, Task>,
    ) -> (Vec<ScheduledTask>, Vec<TickLog>) {
        let mut final_schedule = Vec::new();
        let mut tick_logs = Vec::new();

        // Count of processor units for each type
        let mut resources_count: HashMap<OperationType, usize> = HashMap::new();
        resources_count.insert(OperationType::Add, self.configuration.processors.add);
        resources_count.insert(OperationType::Sub, self.configuration.processors.sub);
        resources_count.insert(OperationType::Mul, self.configuration.processors.mul);
        resources_count.insert(OperationType::Div, self.configuration.processors.div);

        // Track when each unit is ready for NEW input.
        // Map: OperationType -> Vector of "Ready at Tick" for each unit index.
        let mut unit_next_input_at: HashMap<OperationType, Vec<usize>> = HashMap::new();
        for (op, &count) in &resources_count {
            unit_next_input_at.insert(*op, vec![0; count]);
        }

        let mut task_finish_time: HashMap<usize, usize> = HashMap::new();
        let mut scheduled_tasks_ids: HashSet<usize> = HashSet::new();

        // Process Load operations (latency 0) immediately
        for task in tasks.values() {
            if task.operation_type == OperationType::Load {
                task_finish_time.insert(task.id, 0);
                scheduled_tasks_ids.insert(task.id);
            }
        }

        let mut current_tick = 0;

        // Track currently running tasks for logging purposes: (TaskID, StartTime, EndTime, UnitIndex, OpType)
        let mut active_computations: Vec<(usize, usize, usize, usize, OperationType)> =
            Vec::new();

        loop {
            // Clean up finished tasks from active list (just for housekeeping)
            active_computations.retain(|&(_, _, end, _, _)| end > current_tick);

            // Break if all tasks are scheduled and no computations are running
            if scheduled_tasks_ids.len() == tasks.len() && active_computations.is_empty()
            {
                break;
            }

            // --- 1. Find Ready Tasks ---
            // A task is ready if all dependencies finished at or before current_tick.
            let mut ready_queue: Vec<&Task> = tasks
                .values()
                .filter(|t| !scheduled_tasks_ids.contains(&t.id))
                .filter(|t| {
                    t.dependencies.iter().all(|dep_id| {
                        task_finish_time
                            .get(dep_id)
                            .map(|&t| t <= current_tick)
                            .unwrap_or(false)
                    })
                })
                .collect();

            // Heuristic: Prefer tasks with lower Rank (closer to leaves usually means data ready)
            // or Higher Cost. Here we sort by Rank ASC, then Latency DESC.
            ready_queue.sort_by(|a, b| {
                if a.rank != b.rank {
                    a.rank.cmp(&b.rank)
                } else {
                    let cost_a = a.operation_type.latency(&self.configuration.time);
                    let cost_b = b.operation_type.latency(&self.configuration.time);
                    cost_b.cmp(&cost_a)
                }
            });

            // --- 2. Log State Snapshot (Start of Tick) ---
            let mut current_tick_log = TickLog {
                tick: current_tick,
                pipelines_state: HashMap::new(),
                ready_queue: ready_queue.iter().map(|t| t.display_name.clone()).collect(),
            };

            // --- 3. Schedule Ready Tasks ---
            for task in ready_queue {
                if task.operation_type == OperationType::Load {
                    continue;
                }

                if let Some(units) = unit_next_input_at.get_mut(&task.operation_type) {
                    // Try to find a unit that can accept input NOW (<= current_tick)
                    if let Some(unit_idx) =
                        units.iter().position(|&ready_at| ready_at <= current_tick)
                    {
                        let latency =
                            task.operation_type.latency(&self.configuration.time);
                        let start = current_tick;
                        let end = start + latency;

                        // Pipelining: Unit is ready for NEXT task after 1 tick
                        units[unit_idx] = start + 1;

                        final_schedule.push(ScheduledTask {
                            task_id: task.id,
                            name: task.display_name.clone(),
                            start_time: start,
                            end_time: end,
                            processor_type: task.operation_type,
                            processor_index: unit_idx,
                        });

                        scheduled_tasks_ids.insert(task.id);
                        task_finish_time.insert(task.id, end);

                        active_computations.push((
                            task.id,
                            start,
                            end,
                            unit_idx,
                            task.operation_type,
                        ));
                    }
                }
            }

            // --- 4. Populate Pipeline Status for Log ---
            // We iterate over every processor unit to report its status.
            for (op_type, units) in &unit_next_input_at {
                if *op_type == OperationType::Load {
                    continue;
                }

                for (idx, _) in units.iter().enumerate() {
                    // Find all tasks currently executing in this specific unit instance
                    // A task is in the pipeline if: start_time <= current_tick < end_time
                    let tasks_in_pipeline: Vec<_> = active_computations
                        .iter()
                        .filter(|&&(_, start, end, u_idx, op)| {
                            op == *op_type
                                && u_idx == idx
                                && start <= current_tick
                                && current_tick < end
                        })
                        .collect();

                    let proc_name = format!("{} #{}", op_type, idx + 1);

                    if tasks_in_pipeline.is_empty() {
                        current_tick_log
                            .pipelines_state
                            .insert(proc_name, "Idle".to_string());
                    } else {
                        // Build string like "[Stage 3: a+b] [Stage 1: c*d]"
                        // Current Stage = current_tick - start_time + 1
                        // Sort by start time to show flow order (earlier start = later stage)
                        let mut status_parts = Vec::new();
                        // Sort tasks so highest stage is first (visually intuitive)
                        let mut sorted_tasks = tasks_in_pipeline.clone();
                        sorted_tasks.sort_by_key(|&&(_, start, _, _, _)| start);

                        for &(t_id, start, _, _, _) in sorted_tasks {
                            let current_stage = current_tick - start + 1;
                            let task_name = &tasks[&t_id].display_name;

                            // Shorten name if too long
                            let short_name = if task_name.len() > 15 {
                                format!("{}...", &task_name[0..12])
                            } else {
                                task_name.clone()
                            };

                            status_parts.push(format!(
                                "[Stage {}: {}]",
                                current_stage, short_name
                            ));
                        }

                        current_tick_log
                            .pipelines_state
                            .insert(proc_name, status_parts.join(" "));
                    }
                }
            }

            tick_logs.push(current_tick_log);
            current_tick += 1;

            // Safety break loop
            if current_tick > 10000 {
                break;
            }
        }

        (final_schedule, tick_logs)
    }
}

impl Reporter {
    pub fn pcs_simulation(&self, result: &SimulationResult) -> String {
        let mut buffer = StringBuffer::default();

        buffer.add_line("Parallel Pipelined System Simulation".to_string());
        buffer.add_line("-".repeat(100));

        let conf = &result.configuration;
        buffer.add_line(format!(
            "Configuration: ADD: {} ({}t), SUB: {} ({}t), MUL: {} ({}t), DIV: {} ({}t)",
            conf.processors.add,
            conf.time.add,
            conf.processors.sub,
            conf.time.sub,
            conf.processors.mul,
            conf.time.mul,
            conf.processors.div,
            conf.time.div
        ));
        buffer.add_line("-".repeat(100));

        // Metrics
        buffer.add_line(format!(
            "T1 (Seq): {:<5} | Tp (Par): {:<5} | Speedup: {:<.4} | Efficiency: {:<.4}",
            result.t1, result.tp, result.speedup, result.efficiency
        ));
        buffer.add_line("-".repeat(100));

        // Schedule Table
        buffer.add_line(format!(
            "{:<12} | {:<8} | {:<8} | {:<40}",
            "Processor", "Start", "End", "Operation"
        ));
        buffer.add_line("-".repeat(100));

        let mut sorted_schedule = result.schedule.clone();
        sorted_schedule.sort_by_key(|t| t.start_time);

        for task in sorted_schedule {
            if task.processor_type == OperationType::Load {
                continue;
            }
            buffer.add_line(format!(
                "{:<12} | {:<8} | {:<8} | {:<40}",
                format!("{} #{}", task.processor_type, task.processor_index + 1),
                task.start_time,
                task.end_time,
                task.name
            ));
        }

        buffer.add_line("\nDetailed Pipeline Log (Tick-by-Tick):".to_string());
        buffer.add_line("-".repeat(100));

        for log in &result.tick_logs {
            // Skip trailing idle logs if queue is empty
            if log.pipelines_state.values().all(|s| s == "Idle")
                && log.ready_queue.is_empty()
            {
                continue;
            }

            buffer.add_line(format!("Tick {:02}:", log.tick + 1));
            if !log.ready_queue.is_empty() {
                buffer.add_line(format!("  Ready Queue: {:?}", log.ready_queue));
            }

            // Sort keys for consistent output order
            let mut keys: Vec<_> = log.pipelines_state.keys().collect();
            keys.sort();

            for key in keys {
                let state = &log.pipelines_state[key];
                // Display even if Idle, or filter out if desired.
                // User requested state for ALL blocks.
                buffer.add_line(format!("  {:<10}: {}", key, state));
            }
            buffer.add_line("".to_string());
        }

        buffer.get()
    }
}
