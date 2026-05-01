use crate::backend::simulation::Simulation;
use rand::RngExt;
use rayon::iter::IntoParallelRefMutIterator;
use rayon::iter::ParallelIterator;
use std::sync::Mutex;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum BugMode {
    #[default]
    None,
    RaceCondition,
    Deadlock,
    OneThreadPerAtom,
}

impl std::fmt::Display for BugMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "Off"),
            Self::RaceCondition => write!(f, "Race Condition"),
            Self::Deadlock => write!(f, "Deadlock"),
            Self::OneThreadPerAtom => write!(f, "1 Atom = 1 Thread"),
        }
    }
}

pub fn tick_race_condition(simulation: &mut Simulation) {
    let border = &simulation.settings.crystal_size;
    let field = &simulation.crystal.field;
    let probabilities = &simulation.settings.atom_movement_probability;

    simulation.crystal.atoms.par_iter_mut().for_each(|atom| {
        let random_value: f64 = atom.rng.random();

        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        if random_value < probabilities.up {
            dy = 1;
        } else if random_value < probabilities.up + probabilities.down {
            dy = -1;
        } else if random_value
            < probabilities.up + probabilities.down + probabilities.left
        {
            dx = -1;
        } else if random_value
            < probabilities.up
                + probabilities.down
                + probabilities.left
                + probabilities.right
        {
            dx = 1;
        }

        let mut new_x = atom.x as i32 + dx;
        let mut new_y = atom.y as i32 + dy;

        if new_x < 0 || new_x >= border.width as i32 {
            new_x = atom.x as i32 - dx;
        }
        if new_y < 0 || new_y >= border.height as i32 {
            new_y = atom.y as i32 - dy;
        }

        if new_x != atom.x as i32 || new_y != atom.y as i32 {
            let old_cell_id = atom.y * border.width + atom.x;
            let new_cell_id = (new_y as usize) * border.width + (new_x as usize);

            // RACE CONDITION BUG
            // Threads are reading old value simultaneously, and then
            // clearing results of each other while writing

            // Subtracting
            let old_val = field[old_cell_id].load(Ordering::Relaxed);
            field[old_cell_id].store(old_val.saturating_sub(1), Ordering::Relaxed);

            // Adding
            let new_val = field[new_cell_id].load(Ordering::Relaxed);
            field[new_cell_id].store(new_val + 1, Ordering::Relaxed);

            atom.x = new_x as usize;
            atom.y = new_y as usize;
        }
    });
}

#[allow(clippy::unwrap_used)]
pub fn tick_deadlock(simulation: &mut Simulation) {
    let border = &simulation.settings.crystal_size;
    let field = &simulation.crystal.field;
    let probabilities = &simulation.settings.atom_movement_probability;

    let lock_a = Mutex::new(());
    let lock_b = Mutex::new(());

    simulation.crystal.atoms.par_iter_mut().for_each(|atom| {
        // DEADLOCK BUG
        if atom.x % 2 == 0 {
            // Thread getting A, waiting for B
            let _guard_a = lock_a.lock().unwrap();
            thread::sleep(Duration::from_micros(50));
            let _guard_b = lock_b.lock().unwrap();
        } else {
            // Thread getting B, waiting for A
            let _guard_b = lock_b.lock().unwrap();
            thread::sleep(Duration::from_micros(50));
            let _guard_a = lock_a.lock().unwrap();
        }

        let random_value: f64 = atom.rng.random();

        let mut dx: i32 = 0;
        let mut dy: i32 = 0;

        if random_value < probabilities.up {
            dy = 1;
        } else if random_value < probabilities.up + probabilities.down {
            dy = -1;
        } else if random_value
            < probabilities.up + probabilities.down + probabilities.left
        {
            dx = -1;
        } else if random_value
            < probabilities.up
                + probabilities.down
                + probabilities.left
                + probabilities.right
        {
            dx = 1;
        }

        let mut new_x = atom.x as i32 + dx;
        let mut new_y = atom.y as i32 + dy;

        if new_x < 0 || new_x >= border.width as i32 {
            new_x = atom.x as i32 - dx;
        }
        if new_y < 0 || new_y >= border.height as i32 {
            new_y = atom.y as i32 - dy;
        }

        if new_x != atom.x as i32 || new_y != atom.y as i32 {
            let old_cell_id = atom.y * border.width + atom.x;
            let new_cell_id = (new_y as usize) * border.width + (new_x as usize);

            field[old_cell_id].fetch_sub(1, Ordering::Relaxed);
            field[new_cell_id].fetch_add(1, Ordering::Relaxed);

            atom.x = new_x as usize;
            atom.y = new_y as usize;
        }
    });
}

pub fn tick_one_thread_per_atom(simulation: &mut Simulation) {
    let border = &simulation.settings.crystal_size;
    let field = &simulation.crystal.field;
    let probabilities = &simulation.settings.atom_movement_probability;

    // Using scoped threads to allow new threads
    // to safely borrow field and atom without 'static lifetime
    thread::scope(|s| {
        for atom in simulation.crystal.atoms.iter_mut() {
            // Spawning threads.. oh
            s.spawn(move || {
                let random_value: f64 = atom.rng.random();

                let mut dx: i32 = 0;
                let mut dy: i32 = 0;

                if random_value < probabilities.up {
                    dy = 1;
                } else if random_value < probabilities.up + probabilities.down {
                    dy = -1;
                } else if random_value
                    < probabilities.up + probabilities.down + probabilities.left
                {
                    dx = -1;
                } else if random_value
                    < probabilities.up
                        + probabilities.down
                        + probabilities.left
                        + probabilities.right
                {
                    dx = 1;
                }

                let mut new_x = atom.x as i32 + dx;
                let mut new_y = atom.y as i32 + dy;

                if new_x < 0 || new_x >= border.width as i32 {
                    new_x = atom.x as i32 - dx;
                }
                if new_y < 0 || new_y >= border.height as i32 {
                    new_y = atom.y as i32 - dy;
                }

                if new_x != atom.x as i32 || new_y != atom.y as i32 {
                    let old_cell_id = atom.y * border.width + atom.x;
                    let new_cell_id = (new_y as usize) * border.width + (new_x as usize);

                    field[old_cell_id].fetch_sub(1, Ordering::Relaxed);
                    field[new_cell_id].fetch_add(1, Ordering::Relaxed);

                    atom.x = new_x as usize;
                    atom.y = new_y as usize;
                }
            });
        }
    });
}
