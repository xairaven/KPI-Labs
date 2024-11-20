use crate::config::Config;
use crate::context::Context;
use crate::services::local_data;
use crate::ui::windows;
use egui::ThemePreference;

pub struct App {
    pub context: Context,
    pub config: Config,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>, config: Config) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        const THEME: ThemePreference = ThemePreference::Dark;

        cc.egui_ctx
            .options_mut(|options| options.theme_preference = THEME);

        let local_data = local_data::get("local_data.json").unwrap_or_else(|err| {
            eprintln!("Error occured while loading local_data.json: {}", err);
            std::process::exit(1);
        });

        Self {
            context: Context {
                local_data,
                ..Default::default()
            },
            config,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            windows::main::show(&self.config, &mut self.context, ui);
        });
    }
}
