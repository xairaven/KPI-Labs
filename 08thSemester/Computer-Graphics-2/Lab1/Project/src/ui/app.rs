use crate::config::Config;
use crate::state::State;
use crate::ui;
use crate::ui::components::canvas::Canvas;
use crate::ui::components::settings::Settings;
use crate::ui::modals::Modal;

#[derive(Default)]
pub struct App {
    pub canvas: Canvas,
    pub settings: Settings,

    pub state: State,

    modals: Vec<Box<dyn Modal>>,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        cc.egui_ctx
            .options_mut(|options| options.theme_preference = config.theme);
        Default::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui::core::show(self, ui, ctx);

            // Getting modals from the channels (in context).
            if let Ok(modal) = self.state.modals_rx.try_recv() {
                self.modals.push(modal);
            }

            // Showing modals.
            self.show_opened_modals(ui);
        });
    }
}

impl App {
    fn show_opened_modals(&mut self, ui: &egui::Ui) {
        let mut closed_modals: Vec<usize> = vec![];

        for (index, modal) in self.modals.iter_mut().enumerate() {
            modal.show(&mut self.state, ui);

            if modal.is_closed() {
                closed_modals.push(index);
            }
        }

        closed_modals.iter().for_each(|index| {
            self.modals.remove(*index);
        });
    }
}
