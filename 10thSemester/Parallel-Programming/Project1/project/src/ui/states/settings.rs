use crate::backend::bugs::BugMode;
use crate::backend::crystal::{AtomMovementProbability, CrystalSize};
use crate::backend::simulation::{SimulationError, SimulationSettings};
use rand::RngExt;

#[derive(Debug, Clone)]
pub struct SimulationSettingsUi {
    pub atoms_amount: usize,
    pub time_minutes: usize,
    pub time_seconds: usize,
    pub delay_ms: usize,
    pub sampling_period_seconds: f64,

    pub probability_up: f64,
    pub probability_down: f64,
    pub probability_left: f64,
    pub probability_right: f64,

    pub is_seed_enabled: bool,
    pub seed: String,

    pub border_width: usize,
    pub border_height: usize,

    pub bug_mode: BugMode,
}

impl SimulationSettingsUi {
    pub fn probabilities_sum(&self) -> f64 {
        self.probability_up
            + self.probability_down
            + self.probability_left
            + self.probability_right
    }

    pub fn are_probabilities_valid(&self) -> bool {
        let sum = self.probabilities_sum();

        sum.is_sign_positive() && sum <= 1.0
    }

    pub fn generate_seed(&mut self) {
        let mut rng = rand::rng();
        self.seed = rng.random_range(0..usize::MAX).to_string();
    }

    pub fn generate_random_probabilities(&mut self) {
        let mut rng = rand::rng();

        // Generating 3 cut points on the segment from 0 to 100
        let mut cuts = [
            rng.random_range(0..=100),
            rng.random_range(0..=100),
            rng.random_range(0..=100),
        ];

        // Sorting the cut points to ensure they are in the correct order
        cuts.sort_unstable();

        // Calculating the length of each of the 4 segments formed by the cut points
        let p_up = cuts[0];
        let p_down = cuts[1] - cuts[0];
        let p_left = cuts[2] - cuts[1];
        let p_right = 100 - cuts[2];

        self.probability_up = p_up as f64 / 100.0;
        self.probability_down = p_down as f64 / 100.0;
        self.probability_left = p_left as f64 / 100.0;
        self.probability_right = p_right as f64 / 100.0;
    }

    pub fn reset_probabilities(&mut self) {
        self.probability_up = 0.0;
        self.probability_down = 0.0;
        self.probability_left = 0.0;
        self.probability_right = 0.0;
    }
}

impl Default for SimulationSettingsUi {
    fn default() -> Self {
        Self {
            atoms_amount: 100,
            time_minutes: 0,
            time_seconds: 10,
            delay_ms: 500,
            sampling_period_seconds: 1.0,

            probability_up: 0.0,
            probability_down: 0.0,
            probability_left: 0.0,
            probability_right: 0.0,

            is_seed_enabled: false,
            seed: String::from("0"),

            border_width: 10,
            border_height: 10,

            bug_mode: BugMode::default(),
        }
    }
}

impl TryFrom<SimulationSettingsUi> for SimulationSettings {
    type Error = SimulationError;

    fn try_from(value: SimulationSettingsUi) -> Result<Self, Self::Error> {
        if !value.are_probabilities_valid() {
            return Err(SimulationError::BadProbabilities);
        }
        let seed = if value.is_seed_enabled {
            let seed: u64 = value.seed.parse().map_err(SimulationError::BadSeed)?;
            Some(seed)
        } else {
            None
        };

        Ok(Self {
            atoms_amount: value.atoms_amount,
            time_seconds: value.time_minutes * 60 + value.time_seconds,
            sampling_period_seconds: value.sampling_period_seconds,
            delay_ms: value.delay_ms,
            seed,
            atom_movement_probability: AtomMovementProbability {
                up: value.probability_up,
                down: value.probability_down,
                left: value.probability_left,
                right: value.probability_right,
            },
            crystal_size: CrystalSize {
                width: value.border_width + 1,
                height: value.border_height + 1,
            },
            bug_mode: value.bug_mode,
        })
    }
}
