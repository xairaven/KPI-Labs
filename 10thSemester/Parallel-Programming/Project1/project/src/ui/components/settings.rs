use crate::backend::bugs::BugMode;
use crate::backend::commands::UiCommand;
use crate::backend::simulation;
use crate::backend::simulation::{SimulationError, SimulationSettings};
use crate::context::Context;
use crate::graphics;
use crate::ui::controls::drag_value::DragValueNotifiable;
use crate::ui::modals::error::ErrorModal;
use crate::ui::states::player::ViewMode;
use eframe::epaint::Color32;
use egui::{Button, DragValue, Grid, Panel, RichText, ScrollArea, TextEdit};

#[derive(Debug)]
pub struct SettingsComponent {
    width: f32,
}

impl Default for SettingsComponent {
    fn default() -> Self {
        Self { width: 250.0 }
    }
}

impl SettingsComponent {
    pub fn show(&mut self, ui: &mut egui::Ui, context: &mut Context) {
        Panel::right("SETTINGS_PANEL")
            .resizable(false)
            .default_size(self.width)
            .min_size(self.width)
            .max_size(self.width)
            .show_separator_line(true)
            .show_inside(ui, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.vertical_centered_justified(|ui| {
                        ui.heading(RichText::new("Settings").color(Color32::WHITE));
                    });
                    ui.add_space(10.0);

                    self.simulation_settings(ui, context);

                    self.separator(ui);

                    self.simulation_player(ui, context);

                    self.separator(ui);

                    self.performance(ui, context);

                    self.separator(ui);

                    self.ui_settings(ui, context);
                });
            });
    }

    fn performance(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            self.header(ui, "System Performance");
        });

        let metrics = &context.performance_monitor.current_metrics;

        ui.label(format!(
            "CPU Global Usage: {:.1}%",
            metrics.global_cpu_usage
        ));
        ui.label(format!(
            "RAM: {:.0} / {:.0} MB",
            metrics.memory_used_mb, metrics.memory_total_mb
        ));

        ui.collapsing("CPUs Info", |ui| {
            Grid::new("CPUs").num_columns(2).show(ui, |ui| {
                for unit in &metrics.cpus_info {
                    ui.label(&unit.name);
                    ui.label(format!("{:.1}%", unit.usage));
                    ui.end_row();
                }
            })
        });
    }

    fn ui_settings(&self, ui: &mut egui::Ui, context: &mut Context) {
        ui.vertical_centered_justified(|ui| {
            self.header(ui, "UI Settings");
        });

        ui.horizontal(|ui| {
            ui.label("Pixels on Centimeter:");
            ui.add(
                DragValue::new(&mut context.viewport.geometry.pixels_per_centimeter)
                    .speed(1)
                    .range(graphics::PX_PER_CM_RANGE),
            );
        });

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset").clicked() {
                context.viewport.geometry.reset_pixels_per_centimeter();
            }
        });

        Grid::new("UI_GRID_SETTINGS").num_columns(2).show(ui, |ui| {
            ui.checkbox(&mut context.ui_state.grid.is_enabled, "Grid;");
            ui.checkbox(&mut context.ui_state.grid.are_axes_enabled, "Axes;");
            ui.end_row();
        });

        Grid::new("PAN_ZOOM_SETTINGS")
            .num_columns(2)
            .show(ui, |ui| {
                ui.checkbox(&mut context.viewport.config.is_pannable, "Pan;");
                ui.checkbox(&mut context.viewport.config.is_zoomable, "Zoom");

                ui.end_row();
            });

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset Pan").clicked() {
                context.viewport.geometry.reset_offset();
            }
        });

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset all to defaults").clicked() {
                context.reset();
            }
        });
    }

    fn simulation_settings(&self, ui: &mut egui::Ui, context: &mut Context) {
        self.header(ui, "Simulation Settings");

        let settings = &mut context.ui_state.simulation_settings;
        let commands_tx = &context.ui_commands_tx;

        Grid::new("SimulationSettings")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Border Width:");
                DragValueNotifiable::new(&mut settings.border_width)
                    .speed(1)
                    .range(simulation::ranges::BORDER)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Border Height:");
                DragValueNotifiable::new(&mut settings.border_height)
                    .speed(1)
                    .range(simulation::ranges::BORDER)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Atoms Amount:");
                DragValueNotifiable::new(&mut settings.atoms_amount)
                    .speed(1)
                    .range(simulation::ranges::ATOMS_AMOUNT)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Sampling Period:");
                DragValueNotifiable::new(&mut settings.sampling_period_seconds)
                    .speed(0.1)
                    .range(simulation::ranges::SAMPLING)
                    .suffix(" sec.")
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Delay in drawing:");
                DragValueNotifiable::new(&mut settings.delay_ms)
                    .speed(1)
                    .suffix(" ms.")
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label(RichText::new("Bug Mode:").color(Color32::YELLOW));
                egui::ComboBox::from_id_salt("bug_mode_combobox")
                    .selected_text(settings.bug_mode.to_string())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut settings.bug_mode, BugMode::None, "Off");
                        ui.selectable_value(
                            &mut settings.bug_mode,
                            BugMode::RaceCondition,
                            "Race Condition",
                        );
                        ui.selectable_value(
                            &mut settings.bug_mode,
                            BugMode::Deadlock,
                            "Deadlock",
                        );
                        ui.selectable_value(
                            &mut settings.bug_mode,
                            BugMode::OneThreadPerAtom,
                            "1 Atom = 1 Thread",
                        );
                    });
                ui.end_row();

                ui.label("Seed Enabled:");
                ui.checkbox(&mut settings.is_seed_enabled, "");
                ui.end_row();

                if settings.is_seed_enabled {
                    ui.label("Seed:");
                    ui.add(TextEdit::singleline(&mut settings.seed).desired_width(100.0));
                    ui.end_row();
                }
            });
        ui.vertical_centered_justified(|ui| {
            if settings.is_seed_enabled && ui.button("Generate Seed").clicked() {
                settings.generate_seed();
            }
        });

        ui.horizontal(|ui| {
            ui.label("Time:");
            DragValueNotifiable::new(&mut settings.time_minutes)
                .speed(1)
                .range(simulation::ranges::TIME)
                .tx(commands_tx.clone())
                .command(UiCommand::StopSimulation)
                .show(ui);
            ui.label(":");
            DragValueNotifiable::new(&mut settings.time_seconds)
                .speed(1)
                .range(simulation::ranges::TIME)
                .tx(commands_tx.clone())
                .command(UiCommand::StopSimulation)
                .show(ui);
        });

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            ui.label("Atom Movement Probabilities");
        });
        ui.add_space(5.0);
        Grid::new("AtomProbabilities")
            .num_columns(2)
            .show(ui, |ui| {
                ui.label("Up:");
                DragValueNotifiable::new(&mut settings.probability_up)
                    .speed(0.01)
                    .range(simulation::ranges::MOVEMENT_PROBABILITY)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Left:");
                DragValueNotifiable::new(&mut settings.probability_left)
                    .speed(0.01)
                    .range(simulation::ranges::MOVEMENT_PROBABILITY)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Right:");
                DragValueNotifiable::new(&mut settings.probability_right)
                    .speed(0.01)
                    .range(simulation::ranges::MOVEMENT_PROBABILITY)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();

                ui.label("Down:");
                DragValueNotifiable::new(&mut settings.probability_down)
                    .speed(0.01)
                    .range(simulation::ranges::MOVEMENT_PROBABILITY)
                    .tx(commands_tx.clone())
                    .command(UiCommand::StopSimulation)
                    .show(ui);
                ui.end_row();
            });

        if !settings.are_probabilities_valid() {
            ui.add_space(5.0);
            ui.vertical_centered_justified(|ui| {
                ui.label(
                    RichText::new("Probabilities must sum up to 1.").color(Color32::RED),
                );
            });
        }

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Generate Probabilities").clicked() {
                settings.generate_random_probabilities();
            }
        });

        ui.add_space(5.0);

        ui.vertical_centered_justified(|ui| {
            if ui.button("Reset Probabilities").clicked() {
                settings.reset_probabilities();
            }
        });
    }

    fn simulation_player(&self, ui: &mut egui::Ui, context: &mut Context) {
        self.header(ui, "Simulation Player");
        let player = &mut context.ui_state.player;

        Grid::new("Player").num_columns(3).show(ui, |ui| {
            ui.label("Mode:");
            egui::ComboBox::from_label("")
                .selected_text(player.mode.to_string())
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut player.mode,
                        ViewMode::RealTime,
                        "Real-Time",
                    );
                    ui.selectable_value(&mut player.mode, ViewMode::Snapshot, "Snapshot");
                });
            ui.end_row();
        });

        match player.mode {
            ViewMode::RealTime => self.realtime_mode(ui, context),
            ViewMode::Snapshot => self.snapshot_mode(ui, context),
        }
    }

    fn realtime_mode(&self, ui: &mut egui::Ui, context: &mut Context) {
        let player = &mut context.ui_state.player;

        Grid::new("RealTimePlayer").num_columns(2).show(ui, |ui| {
            ui.label("Status:");
            ui.label(match player.real_time.is_enabled {
                true => RichText::new("Running").color(Color32::GREEN),
                false => RichText::new("Stopped").color(Color32::RED),
            });
            ui.end_row();
            ui.label("Time:");
            ui.label(player.real_time.time());
            ui.end_row();

            if let Some(atoms_amount) = player.real_time.total_atoms() {
                ui.label("Total Atoms:");
                ui.label(atoms_amount.to_string());
                ui.end_row();
            }
        });

        ui.vertical_centered_justified(|ui| match player.real_time.is_enabled {
            true => {
                if ui.button("Stop").clicked() {
                    player.real_time.stop();
                }
            },
            false => {
                if ui.button("Start").clicked() {
                    let settings: Result<SimulationSettings, SimulationError> =
                        context.ui_state.simulation_settings.clone().try_into();
                    match settings {
                        Ok(settings) => {
                            player.finish_real_time();
                            player.history.clear();
                            player.real_time.start(&settings);
                        },
                        Err(err) => {
                            let _ = context
                                .error_modals_tx
                                .try_send(ErrorModal::new(err.into()));
                        },
                    }
                }
            },
        });
    }

    fn snapshot_mode(&self, ui: &mut egui::Ui, context: &mut Context) {
        let visualizer = &mut context.ui_state.player.history;

        if visualizer.is_empty() {
            ui.label(RichText::new("No snapshots available").color(Color32::YELLOW));
            return;
        }

        let current_index = if let Some(value) = visualizer.current_index() {
            *value
        } else {
            return;
        };

        ui.horizontal(|ui| {
            if ui
                .add_enabled(current_index > 0, Button::new("<"))
                .clicked()
            {
                visualizer.left();
            };
            ui.label(format!("{} / {}", current_index + 1, visualizer.len()));
            if ui
                .add_enabled(current_index < visualizer.len() - 1, Button::new(">"))
                .clicked()
            {
                visualizer.right();
            };
            if ui.button("Clear").clicked() {
                visualizer.clear();
            }
        });

        let snapshot = if let Some(value) = visualizer.get(current_index) {
            value
        } else {
            return;
        };

        ui.label(format!("Total atoms: {}", snapshot.total_atoms));
    }

    fn separator(&self, ui: &mut egui::Ui) {
        ui.add_space(10.0);
        ui.separator();
        ui.add_space(10.0);
    }

    fn header(&self, ui: &mut egui::Ui, header: &str) {
        ui.vertical_centered_justified(|ui| {
            ui.label(RichText::new(header).size(14.0));
        });

        ui.add_space(5.0);
    }
}
