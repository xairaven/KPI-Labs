use crate::graphics::screen::ScreenParams;
use egui::Pos2;
use egui::epaint::CircleShape;
use egui::{Color32, Shape, Stroke};
use nalgebra::SMatrix;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,

    // Debug fields:
    pub converted_to_screen: bool,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            converted_to_screen: false,
        }
    }

    pub fn as_vector(&self) -> SMatrix<f32, 1, 3> {
        SMatrix::<f32, 1, 3>::new(self.x, self.y, 1.0)
    }

    pub fn as_pos2(&self) -> Pos2 {
        Pos2::from([self.x, self.y])
    }

    pub fn from_pos2(pos: Pos2) -> Self {
        Self {
            x: pos.x,
            y: pos.y,
            converted_to_screen: false,
        }
    }

    pub fn as_screen(&self, screen_params: ScreenParams) -> Self {
        screen_params.point_cm_to_px(*self)
    }

    pub fn as_shape(&self, radius: f32, color: Color32) -> Shape {
        Shape::circle_filled(self.as_pos2(), radius, color)
    }

    pub fn as_dot(&self, radius: f32, fill: Color32, stroke: Stroke) -> Shape {
        let mut shape = CircleShape::filled(self.as_pos2(), radius, fill);
        shape.stroke = stroke;

        Shape::Circle(shape)
    }

    pub fn with_converted_checked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: true,
        }
    }

    pub fn with_converted_unchecked(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
            converted_to_screen: false,
        }
    }
}
