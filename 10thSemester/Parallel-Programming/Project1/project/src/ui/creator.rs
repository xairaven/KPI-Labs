use crate::backend::commands::EngineEvent;
use crate::config::Config;
use crate::context::Context;
use crate::ui::modals::ModalsHandler;
use crate::ui::workspace::Workspace;
use eframe::Frame;
use egui::{CentralPanel, Ui};

pub struct AppCreator {
    pub context: Context,
    pub workspace: Workspace,

    modals_handler: ModalsHandler,
}

impl AppCreator {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        Self::set_theme(cc, &config);

        let context = Context::new(config);
        let workspace = Workspace::new(&context);

        Self {
            context,
            workspace,
            modals_handler: ModalsHandler::default(),
        }
    }

    fn set_theme(cc: &eframe::CreationContext<'_>, config: &Config) {
        cc.egui_ctx.set_theme(config.theme);
    }
}

impl eframe::App for AppCreator {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        CentralPanel::default().show_inside(ui, |ui| {
            self.workspace.show(ui, &mut self.context);

            self.modals_handler.handle_errors(ui, &self.context);
        });

        // Engine event handling
        while let Ok(event) = self.context.engine_event_rx.try_recv() {
            match event {
                EngineEvent::AlgorithmPassed(snapshot) => {
                    self.context.ui_state.player.pass_real_snapshot(snapshot);
                },
                EngineEvent::Snapshot(snapshot) => {
                    self.context.ui_state.player.history.add(snapshot);
                },
                EngineEvent::SimulationFinished => {
                    self.context.ui_state.player.finish_real_time();
                },
            }
        }

        if self.context.performance_monitor.update().is_err() {
            ui.close();
        }
        ui.request_repaint();
    }
}
