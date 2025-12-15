use crate::config::Config;
use crate::context::Context;
use crate::ui::components::main::MainComponent;
use crate::ui::components::side::SideComponent;
use crate::ui::modals::Modal;
use crate::ui::modals::error::ErrorModal;
use egui::{CentralPanel, SidePanel};

pub struct App {
    context: Context,

    main_component: MainComponent,
    side_panel: SideComponent,

    errors: Vec<ErrorModal>,
}

impl App {
    pub fn new(_: &eframe::CreationContext<'_>, config: Config) -> Self {
        let context = Context::new(config);

        Self {
            context,

            main_component: Default::default(),
            side_panel: Default::default(),

            errors: vec![],
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            SidePanel::right("SETTINGS_PANEL")
                .resizable(false)
                .min_width(ui.available_width() / 4.0)
                .max_width(ui.available_width() / 4.0)
                .show_separator_line(true)
                .show_inside(ui, |ui| {
                    self.side_panel.show(&mut self.context, ui);
                });

            CentralPanel::default().show_inside(ui, |ui| {
                self.main_component.show(&mut self.context, ui);
            });

            // Getting modals from the channels (in context).
            if let Ok(modal) = self.context.ui.errors_rx.try_recv() {
                self.errors.push(modal);
            }

            // Showing modals.
            self.show_opened_modals(ui);
        });

        ctx.request_repaint();
    }
}

impl App {
    fn show_opened_modals(&mut self, ui: &egui::Ui) {
        let mut closed_modals: Vec<usize> = vec![];

        for (index, modal) in self.errors.iter_mut().enumerate() {
            modal.show(ui, &mut self.context);

            if modal.is_closed() {
                closed_modals.push(index);
            }
        }

        closed_modals.iter().for_each(|index| {
            self.errors.remove(*index);
        });
    }
}
