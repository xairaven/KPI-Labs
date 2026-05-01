use crate::backend::simulation::SimulationSettings;
use crate::backend::snapshot::CrystalSnapshot;

#[derive(Debug, Clone)]
pub enum UiCommand {
    StartSimulation(SimulationSettings),
    StopSimulation,
}

#[derive(Debug, Clone)]
pub enum EngineEvent {
    AlgorithmPassed(CrystalSnapshot),
    Snapshot(CrystalSnapshot),
    SimulationFinished,
}
