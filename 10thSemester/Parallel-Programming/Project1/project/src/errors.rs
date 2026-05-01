use crate::backend::simulation::SimulationError;
use crate::config::ConfigError;
use crate::logger::LoggerError;
use crate::ui::GraphicsBackendError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Configuration. {0}")]
    Config(#[from] ConfigError),

    #[error("Graphics Backend. {0}")]
    GraphicsBackend(#[from] GraphicsBackendError),

    #[error("Logger. {0}")]
    Logger(#[from] LoggerError),

    #[error("Simulation. {0}")]
    Simulation(#[from] SimulationError),
}
