use crate::compiler::context::CompilerContext;
use crate::config::Config;
use crate::errors::Error;
use crate::ui::context::UIContext;
use crate::ui::modals::error::ErrorModal;

pub struct Context {
    pub compiler: CompilerContext,
    pub ui: UIContext,

    pub config: Config,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            compiler: CompilerContext::new(&config),
            ui: UIContext::new(&config),

            config,
        }
    }

    pub fn save_config(&mut self) {
        self.config.pretty_output = self.compiler.pretty_output;

        if let Err(error) = self.config.save_to_file() {
            let error: Error = error.into();
            ErrorModal::new(error).try_send_by(&self.ui.errors_tx);
        }
    }
}
