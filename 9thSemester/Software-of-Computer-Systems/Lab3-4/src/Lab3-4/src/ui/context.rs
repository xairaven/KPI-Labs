use crate::config::Config;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender, unbounded};

pub struct UIContext {
    pub output: Option<String>,

    pub errors_tx: Sender<ErrorModal>,
    pub errors_rx: Receiver<ErrorModal>,
}

impl UIContext {
    pub fn new(_: &Config) -> Self {
        let (errors_tx, errors_rx) = unbounded::<ErrorModal>();

        Self {
            output: None,
            errors_tx,
            errors_rx,
        }
    }

    pub fn set_output(&mut self, output: String) {
        self.output = Some(output);
    }

    pub fn get_output(&mut self) -> Option<String> {
        self.output.take()
    }
}
