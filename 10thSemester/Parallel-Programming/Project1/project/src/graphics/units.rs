use crate::graphics::Viewport;
use crate::graphics::primitives::point::{Point, PointPixel};
use derive_more::{
    Add, AddAssign, Display, Div, From, Into, Mul, MulAssign, Neg, Rem, Sub,
};
use std::ops::{Deref, DerefMut};

#[derive(
    Add,
    AddAssign,
    MulAssign,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
    From,
    Into,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
)]
pub struct Centimeter(pub f64);

impl Centimeter {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Deref for Centimeter {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Centimeter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(
    Add,
    AddAssign,
    Sub,
    Mul,
    Div,
    Neg,
    Rem,
    From,
    Into,
    Display,
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
)]
pub struct Pixel(pub f64);

impl Pixel {
    pub fn value(&self) -> f64 {
        self.0
    }
}

impl Deref for Pixel {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/* CONVERSION */

impl Centimeter {
    pub fn to_pixels_x(self, viewport: &Viewport) -> Pixel {
        let value = viewport.state.zero_point.x.value()
            + (self.value() * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.x.value();

        Pixel(value)
    }

    pub fn to_pixels_y(self, viewport: &Viewport) -> Pixel {
        let value = viewport.state.zero_point.y.value()
            - (self.value() * viewport.geometry.pixels_per_centimeter)
            + viewport.geometry.offset.y.value();

        Pixel(value)
    }

    pub fn to_pixels_vector_x(self, viewport: &Viewport) -> Pixel {
        Pixel(self.value() * viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_pixels_vector_y(self, viewport: &Viewport) -> Pixel {
        Pixel(-self.value() * viewport.geometry.pixels_per_centimeter)
    }
}

impl Pixel {
    pub fn to_centimeter_vector_x(self, viewport: &Viewport) -> Centimeter {
        Centimeter(self.value() / viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_centimeter_vector_y(self, viewport: &Viewport) -> Centimeter {
        Centimeter(-self.value() / viewport.geometry.pixels_per_centimeter)
    }

    pub fn to_centimeters_x(self, viewport: &Viewport) -> Centimeter {
        let value = (self.value()
            - viewport.state.zero_point.x.value()
            - viewport.geometry.offset.x.value())
            / viewport.geometry.pixels_per_centimeter;

        Centimeter(value)
    }

    pub fn to_centimeters_y(self, viewport: &Viewport) -> Centimeter {
        let value = (-self.value()
            + viewport.state.zero_point.y.value()
            + viewport.geometry.offset.y.value())
            / viewport.geometry.pixels_per_centimeter;

        Centimeter(value)
    }
}

impl Point {
    pub fn to_pixels(self, viewport: &Viewport) -> PointPixel {
        let x = self.x.to_pixels_x(viewport);
        let y = self.y.to_pixels_y(viewport);

        PointPixel { x, y }
    }
}

impl PointPixel {
    pub fn to_centimeters(self, viewport: &Viewport) -> Point {
        let x = self.x.to_centimeters_x(viewport);
        let y = self.y.to_centimeters_y(viewport);

        Point { x, y }
    }
}
