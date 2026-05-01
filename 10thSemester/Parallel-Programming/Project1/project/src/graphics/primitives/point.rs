use crate::graphics::units::{Centimeter, Pixel};
use egui::Pos2;

pub trait Pointable2D: Clone {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: Centimeter,
    pub y: Centimeter,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Centimeter(x),
            y: Centimeter(y),
        }
    }

    pub fn zero() -> Self {
        Self {
            x: Centimeter(0.0),
            y: Centimeter(0.0),
        }
    }
}

impl Pointable2D for Point {
    fn x(&self) -> f64 {
        *self.x
    }

    fn y(&self) -> f64 {
        *self.y
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct PointPixel {
    pub x: Pixel,
    pub y: Pixel,
}

impl PointPixel {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: Pixel(x),
            y: Pixel(y),
        }
    }

    pub const fn zero() -> Self {
        Self {
            x: Pixel(0.0),
            y: Pixel(0.0),
        }
    }
}

impl From<PointPixel> for Pos2 {
    fn from(point: PointPixel) -> Self {
        Pos2::from([point.x.0 as f32, point.y.0 as f32])
    }
}

impl From<Pos2> for PointPixel {
    fn from(pos: Pos2) -> Self {
        Self {
            x: Pixel(pos.x as f64),
            y: Pixel(pos.y as f64),
        }
    }
}

impl Pointable2D for PointPixel {
    fn x(&self) -> f64 {
        *self.x
    }

    fn y(&self) -> f64 {
        *self.y
    }
}
