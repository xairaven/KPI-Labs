use crate::backend::crystal::CrystalSize;
use crate::graphics::primitives::line::Line;
use crate::graphics::primitives::point::Point;
use egui::{Color32, Stroke};

#[derive(Debug)]
pub struct Border {
    m: usize,
    n: usize,

    stroke: Stroke,
}

impl Default for Border {
    fn default() -> Self {
        Self {
            m: 11,
            n: 11,

            stroke: Stroke::new(2.0, Color32::BLACK),
        }
    }
}

impl Border {
    pub fn lines(&self) -> Vec<Line<Point>> {
        let s = self.stroke;

        vec![
            Line::with_coordinates(Point::zero(), Point::new(self.m as f64, 0.0))
                .with_stroke(s),
            Line::with_coordinates(Point::zero(), Point::new(0.0, self.n as f64))
                .with_stroke(s),
            Line::with_coordinates(
                Point::new(self.m as f64, 0.0),
                Point::new(self.m as f64, self.n as f64),
            )
            .with_stroke(s),
            Line::with_coordinates(
                Point::new(0.0, self.n as f64),
                Point::new(self.m as f64, self.n as f64),
            )
            .with_stroke(s),
        ]
    }

    pub fn resize(&mut self, size: &CrystalSize) {
        self.m = size.width - 1;
        self.n = size.height - 1;
    }
}
