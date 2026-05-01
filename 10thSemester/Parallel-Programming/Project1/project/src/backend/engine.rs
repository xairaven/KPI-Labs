use crate::backend::commands::{EngineEvent, UiCommand};
use crate::backend::simulation::Simulation;
use crate::backend::snapshot::CrystalSnapshot;
use crate::ui::modals::error::ErrorModal;
use crossbeam::channel::{Receiver, Sender};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Engine {
    pub simulation: Option<Simulation>,

    pub ui_commands_rx: Receiver<UiCommand>,
    pub events_tx: Sender<EngineEvent>,
    pub _errors_tx: Sender<ErrorModal>,
}

impl Engine {
    pub fn new(
        commands: Receiver<UiCommand>, events: Sender<EngineEvent>,
        errors: Sender<ErrorModal>,
    ) -> Self {
        Self {
            simulation: None,
            ui_commands_rx: commands,
            events_tx: events,
            _errors_tx: errors,
        }
    }

    pub fn run(&mut self) {
        let mut is_running = false;
        let mut last_snapshot_taken_at: Option<Instant> = None;
        let mut last_realtime_sent_at = Instant::now();

        loop {
            while let Ok(command) = self.ui_commands_rx.try_recv() {
                match command {
                    UiCommand::StartSimulation(settings) => {
                        self.simulation = Some(Simulation::new(settings));
                        is_running = true;
                        last_snapshot_taken_at = None;
                    },
                    UiCommand::StopSimulation => {
                        is_running = false;
                        last_snapshot_taken_at = None;
                    },
                }
            }

            if is_running && let Some(simulation) = &mut self.simulation {
                if simulation.start_time.elapsed().as_secs_f64()
                    >= simulation.settings.time_seconds as f64
                {
                    is_running = false;
                    let _ = self.events_tx.send(EngineEvent::SimulationFinished);
                    continue;
                }

                // ALGORITHM PASSING (IMPORTANT)
                simulation.tick();

                let snapshot = CrystalSnapshot::new(&simulation.crystal);
                if last_realtime_sent_at.elapsed().as_millis() >= 16 {
                    let _ = self
                        .events_tx
                        .send(EngineEvent::AlgorithmPassed(snapshot.clone()));

                    last_realtime_sent_at = Instant::now();
                }

                // Delay
                if simulation.settings.delay_ms > 0 {
                    std::thread::sleep(Duration::from_millis(
                        simulation.settings.delay_ms as u64,
                    ));
                }

                // Sampling
                match last_snapshot_taken_at {
                    Some(time)
                        if time.elapsed().as_secs_f64()
                            < simulation.settings.sampling_period_seconds =>
                    {
                        // Pass
                    },
                    _ => {
                        let _ = self.events_tx.send(EngineEvent::Snapshot(snapshot));
                        last_snapshot_taken_at = Some(Instant::now());
                    },
                }
            } else {
                // If thread is on pause, it should sleep to avoid 100% CPU usage
                std::thread::sleep(Duration::from_millis(50));
            }
        }
    }
}
