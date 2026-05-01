use crate::backend::simulation::SimulationSettings;
use rand::SeedableRng;
use rand::rngs::SmallRng;
use std::sync::atomic::AtomicUsize;

#[derive(Debug)]
pub struct Crystal {
    pub atoms: Vec<Atom>,
    pub field: Vec<AtomicUsize>,

    pub size: CrystalSize,
}

impl Crystal {
    pub fn new(settings: &SimulationSettings) -> Self {
        let size = settings.crystal_size.clone();

        let initial_x = size.width / 2;
        let initial_y = size.height / 2;

        let base_seed = settings.seed.unwrap_or_else(rand::random);

        let atoms = (0..settings.atoms_amount)
            .map(|id| {
                // Unique atom seed
                let atom_seed = base_seed.wrapping_add(id as u64);
                // Generator for random
                let rng = SmallRng::seed_from_u64(atom_seed);

                Atom {
                    rng,
                    x: initial_x,
                    y: initial_y,
                }
            })
            .collect();

        let field = (0..size.width * size.height)
            .map(|_| AtomicUsize::new(0))
            .collect();

        let crystal = Self { atoms, field, size };

        // Synchronizing field
        for atom in &crystal.atoms {
            let idx = atom.y * crystal.size.width + atom.x;
            crystal.field[idx].fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        }

        crystal
    }
}

#[derive(Debug, Clone)]
pub struct Atom {
    pub x: usize,
    pub y: usize,
    pub rng: SmallRng,
}

#[derive(Debug, Clone)]
pub struct AtomMovementProbability {
    pub up: f64,
    pub down: f64,
    pub left: f64,
    pub right: f64,
}

#[derive(Debug, Clone)]
pub struct CrystalSize {
    pub width: usize,
    pub height: usize,
}
