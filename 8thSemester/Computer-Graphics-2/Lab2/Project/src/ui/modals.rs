use crate::state::State;

pub trait Modal: Send + Sync {
    fn show(&mut self, state: &mut State, ui: &egui::Ui);
    fn is_closed(&self) -> bool;
}

pub mod message;
