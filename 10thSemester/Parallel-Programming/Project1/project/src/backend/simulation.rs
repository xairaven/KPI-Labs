use crate::backend::bugs;
use crate::backend::bugs::BugMode;
use crate::backend::crystal::{AtomMovementProbability, Crystal, CrystalSize};
use rand::RngExt;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::atomic::Ordering;
use std::time::Instant;
use thiserror::Error;

#[derive(Debug)]
pub struct Simulation {
    pub crystal: Crystal,
    pub settings: SimulationSettings,
    pub start_time: Instant,
}

impl Simulation {
    pub fn new(settings: SimulationSettings) -> Self {
        let crystal = Crystal::new(&settings);
        let start_time = Instant::now();

        Self {
            crystal,
            settings,
            start_time,
        }
    }

    pub fn tick(&mut self) {
        match &self.settings.bug_mode {
            BugMode::RaceCondition => bugs::tick_race_condition(self),
            BugMode::Deadlock => bugs::tick_deadlock(self),
            BugMode::OneThreadPerAtom => bugs::tick_one_thread_per_atom(self),
            BugMode::None => {},
        }

        if !matches!(self.settings.bug_mode, BugMode::None) {
            return;
        }

        // Using new variables for shortening this big names.. oh
        let border = &self.settings.crystal_size;
        let field = &self.crystal.field;
        let probabilities = &self.settings.atom_movement_probability;

        // Parallelism itself!
        self.crystal.atoms.par_iter_mut().for_each(|atom| {
            // Generating number in range 0.0 to 1.0 from personal atom gen
            let random_value: f64 = atom.rng.random();

            // Deciding where atom need to go
            let mut dx: i32 = 0;
            let mut dy: i32 = 0;

            if random_value < probabilities.up {
                dy = 1; // Going UP
            } else if random_value < probabilities.up + probabilities.down {
                dy = -1; // Going DOWN
            } else if random_value
                < probabilities.up + probabilities.down + probabilities.left
            {
                dx = -1; // Going LEFT
            } else if random_value
                < probabilities.up
                    + probabilities.down
                    + probabilities.left
                    + probabilities.right
            {
                dx = 1; // Going RIGHT
            }
            // All other cases (such as the sum of the values < 1.0,
            // and rand_val having lost the excess)
            // set dx = 0 and dy = 0, so that the atom will not go anywhere.

            // New coordinates
            let mut new_x = atom.x as i32 + dx;
            let mut new_y = atom.y as i32 + dy;

            // Boundary check. Strategy - going in other direction
            if new_x < 0 || new_x >= border.width as i32 {
                new_x = atom.x as i32 - dx;
            }
            if new_y < 0 || new_y >= border.height as i32 {
                new_y = atom.y as i32 - dy;
            }

            // Updating field and atoms coordinates, ONLY IF IT'S MOVED
            if new_x != atom.x as i32 || new_y != atom.y as i32 {
                let old_cell_id = atom.y * border.width + atom.x;
                let new_cell_id = (new_y as usize) * border.width + (new_x as usize);

                // Lock-free atom field update
                field[old_cell_id].fetch_sub(1, Ordering::Relaxed);
                field[new_cell_id].fetch_add(1, Ordering::Relaxed);

                // Updating atom coordinates.
                atom.x = new_x as usize;
                atom.y = new_y as usize;
            }
        });
    }
}

#[derive(Debug, Clone)]
pub struct SimulationSettings {
    pub atoms_amount: usize,
    pub time_seconds: usize,
    pub delay_ms: usize,
    pub sampling_period_seconds: f64,
    pub seed: Option<u64>,
    pub atom_movement_probability: AtomMovementProbability,
    pub crystal_size: CrystalSize,
    pub bug_mode: BugMode,
}

pub mod ranges {
    use std::ops::RangeInclusive;

    pub const ATOMS_AMOUNT: RangeInclusive<usize> = 1..=100_000;
    pub const TIME: RangeInclusive<usize> = 0..=59;
    pub const SAMPLING: RangeInclusive<f64> = 0.1..=100.0;
    pub const MOVEMENT_PROBABILITY: RangeInclusive<f64> = 0.0..=1.0;
    pub const BORDER: RangeInclusive<usize> = 5..=100;
}

#[derive(Debug, Error)]
pub enum SimulationError {
    #[error(
        "Probabilities must be non-negative and their sum must be less than or equal to 1."
    )]
    BadProbabilities,

    #[error("Seed must be a positive number. {0}")]
    BadSeed(std::num::ParseIntError),
}
