use crate::graphics::grid::Grid;
use crate::interpolation::state::InterpolationContext;
use crate::ui::modals::Modal;
use crate::ui::styles::colors;
use crossbeam::channel::{Receiver, Sender, unbounded};
use egui::Color32;

pub struct State {
    pub grid: Grid,

    pub interpolation_context: InterpolationContext,
    pub is_curve_enabled: bool,
    pub curve_color: Color32,
    pub is_linear_enabled: bool,
    pub linear_color: Color32,
    pub is_lagrange_enabled: bool,
    pub lagrange_color: Color32,

    pub modals_tx: Sender<Box<dyn Modal>>,
    pub modals_rx: Receiver<Box<dyn Modal>>,
}

impl Default for State {
    fn default() -> Self {
        let (modals_tx, modals_rx) = unbounded::<Box<dyn Modal>>();

        Self {
            grid: Default::default(),
            interpolation_context: Default::default(),

            is_curve_enabled: false,
            curve_color: colors::BLUE,
            is_linear_enabled: false,
            linear_color: colors::PINK,
            is_lagrange_enabled: false,
            lagrange_color: colors::GREEN,

            modals_tx,
            modals_rx,
        }
    }
}
