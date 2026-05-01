use crate::graphics::Viewport;
use crate::graphics::primitives::point::{Point, PointPixel, Pointable2D};
use egui::{Shape, Stroke};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Line<T>
where
    T: Pointable2D,
{
    pub start: T,
    pub end: T,

    pub stroke: Stroke,
}

#[allow(dead_code)]
impl<T> Line<T>
where
    T: Pointable2D,
{
    pub fn new(start: T, end: T, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(
            (self.end.x() - self.start.x()).powf(2.0)
                + (self.end.y() - self.start.y()).powf(2.0),
        )
    }

    pub fn with_coordinates(start: T, end: T) -> Self {
        Self {
            start,
            end,
            stroke: Stroke::default(),
        }
    }

    pub fn with_stroke(self, stroke: Stroke) -> Self {
        Self {
            start: self.start,
            end: self.end,
            stroke,
        }
    }

    pub fn is_transparent(&self) -> bool {
        self.stroke == Stroke::default()
    }
}

impl Line<Point> {
    pub fn to_pixels(self, viewport: &Viewport) -> Line<PointPixel> {
        Line {
            start: self.start.to_pixels(viewport),
            end: self.end.to_pixels(viewport),
            stroke: self.stroke,
        }
    }
}

impl Line<PointPixel> {
    #[allow(dead_code)]
    pub fn to_centimeters(self, viewport: &Viewport) -> Line<Point> {
        Line {
            start: self.start.to_centimeters(viewport),
            end: self.end.to_centimeters(viewport),
            stroke: self.stroke,
        }
    }

    pub fn to_shape(self) -> Shape {
        Shape::line(vec![self.start.into(), self.end.into()], self.stroke)
    }
}
