use crate::backend::crystal::{Crystal, CrystalSize};
use crate::graphics::primitives::dot::{Dot, TooltipMetadata};
use crate::graphics::primitives::point::Point;
use crate::graphics::units::Centimeter;
use egui::Color32;

#[derive(Debug, Clone)]
pub struct CrystalSnapshot {
    pub field: Vec<usize>,
    pub size: CrystalSize,
    pub total_atoms: usize,
}

impl CrystalSnapshot {
    pub fn new(crystal: &Crystal) -> Self {
        let field: Vec<usize> = crystal
            .field
            .iter()
            .map(|c| c.load(std::sync::atomic::Ordering::Relaxed))
            .collect();
        let total_atoms = field.iter().sum();
        let size = crystal.size.clone();

        Self {
            field,
            total_atoms,
            size,
        }
    }

    pub fn dots(&self) -> Vec<Dot> {
        let mut dots = vec![];

        for (id, &count) in self.field.iter().enumerate() {
            if count == 0 {
                continue;
            }

            let x = id % self.size.width;
            let y = id / self.size.width;

            let center = Point::new(x as f64, y as f64);
            let radius = Centimeter(0.3);
            let tooltip_metadata = TooltipMetadata {
                text: format!("Dot [{}; {}].\nAtoms: {}.", x, y, count),
                radius,
                center,
                id,
            };

            let dot = Dot {
                center,
                radius,
                fill: self.coloring(count),
                stroke_color: Color32::BLACK,
                stroke_width: Centimeter(0.05),
                tooltip: Some(tooltip_metadata),
            };

            dots.push(dot);
        }

        dots
    }

    fn coloring(&self, count: usize) -> Color32 {
        if count == 0 {
            // If atom is clear -> then color is transparent.
            return Color32::TRANSPARENT;
        }

        // Uniform distribution: the ratio of atoms in a cell to their total number .max(1)
        // ensures that we never divide by zero (even if total_atoms = 0)
        let intensity = count as f32 / self.total_atoms.max(1) as f32;

        // Forming RGB gradient (Blue -> Green -> Red)
        // If intensity ~ 0.0 (few atoms), color will be pure blue
        // If intensity = 0.5 (50% of all atoms), color will be light green
        // If intensity = 1.0 (100% of atoms in this cell), color will be pure red
        let r = (255.0 * intensity) as u8;
        let b = (255.0 * (1.0 - intensity)) as u8;
        let g = (255.0 * (1.0 - (intensity * 2.0 - 1.0).abs())) as u8;

        Color32::from_rgb(r, g, b)
    }
}
