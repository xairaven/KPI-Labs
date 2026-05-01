use crate::backend::commands::{EngineEvent, UiCommand};
use crate::backend::engine::Engine;
use crate::backend::performance::PerformanceMonitor;
use crate::config::Config;
use crate::graphics::figures::grid::{Grid2D, Grid2DBuilder};
use crate::graphics::units::{Centimeter, Pixel};
use crate::graphics::{Viewport, ViewportGeometry, ViewportState, ZeroPointLocation};
use crate::ui::modals::error::ErrorModal;
use crate::ui::states::player::Player;
use crate::ui::states::settings::SimulationSettingsUi;
use crossbeam::channel::{Receiver, Sender};

#[derive(Debug)]
pub struct Context {
    pub ui_state: UiState,
    pub performance_monitor: PerformanceMonitor,
    pub viewport: Viewport,

    pub config: Config,

    pub ui_commands_tx: Sender<UiCommand>,
    pub engine_event_rx: Receiver<EngineEvent>,
    pub error_modals_tx: Sender<ErrorModal>,
    pub error_modals_rx: Receiver<ErrorModal>,
}

impl Context {
    pub fn new(config: Config) -> Self {
        let (ui_commands_tx, ui_commands_rx) = crossbeam::channel::unbounded();
        let (engine_event_tx, engine_event_rx) = crossbeam::channel::unbounded();
        let (error_modals_tx, error_modals_rx) = crossbeam::channel::unbounded();

        let mut engine =
            Engine::new(ui_commands_rx, engine_event_tx, error_modals_tx.clone());
        std::thread::spawn(move || {
            engine.run();
        });

        Self {
            ui_state: UiState::new(ui_commands_tx.clone()),
            performance_monitor: PerformanceMonitor::new(),

            viewport: Viewport {
                // Default settings like panning, zooming, etc.
                config: Default::default(),
                // Default geometry settings, can be updated by user
                geometry: ViewportGeometry {
                    zero_point_location: ZeroPointLocation::BottomLeftWithOffset {
                        offset: Pixel(50.0),
                    },
                    ..Default::default()
                },
                // Initial viewport state, will be updated when the UI is built
                state: ViewportState::default(),
            },

            config,

            ui_commands_tx,
            engine_event_rx,
            error_modals_tx,
            error_modals_rx,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.config.clone());
    }
}

#[derive(Debug)]
pub struct UiState {
    pub grid: Grid2D,
    pub player: Player,
    pub simulation_settings: SimulationSettingsUi,
}

impl UiState {
    pub fn new(ui_tx: Sender<UiCommand>) -> Self {
        Self {
            player: Player::new(ui_tx),
            grid: Grid2DBuilder::default().with_unit(Centimeter(1.0)).build(),
            simulation_settings: Default::default(),
        }
    }
}
