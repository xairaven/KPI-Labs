use crate::approximation::state::ApproximationState;
use crate::graphics::grid::Grid;
use crate::ui::modals::Modal;
use crate::ui::styles::colors;
use crossbeam::channel::{Receiver, Sender, unbounded};
use egui::Color32;

pub struct State {
    pub grid: Grid,

    pub approximation_state: ApproximationState,
    pub is_curve_enabled: bool,
    pub curve_color: Color32,
    pub is_line_enabled: bool,
    pub linear_color: Color32,
    pub is_parabola_enabled: bool,
    pub quadratic_color: Color32,

    pub modals_tx: Sender<Box<dyn Modal>>,
    pub modals_rx: Receiver<Box<dyn Modal>>,
}

impl Default for State {
    fn default() -> Self {
        let (modals_tx, modals_rx) = unbounded::<Box<dyn Modal>>();

        Self {
            grid: Default::default(),
            approximation_state: Default::default(),

            is_curve_enabled: false,
            curve_color: colors::BLUE,
            is_line_enabled: false,
            linear_color: colors::PINK,
            is_parabola_enabled: false,
            quadratic_color: colors::GREEN,

            modals_tx,
            modals_rx,
        }
    }
}
