use crate::geometry::point2d::Point2D;
use crate::graphics::screen::ScreenParams;
use egui::{Shape, Stroke};

#[derive(Debug, Default, Clone, Copy)]
pub struct Line2D {
    pub start: Point2D,
    pub end: Point2D,

    pub stroke: Stroke,
}

impl Line2D {
    pub fn new(start: Point2D, end: Point2D, stroke: Stroke) -> Self {
        Self { start, end, stroke }
    }

    pub fn to_shape(self) -> Shape {
        Shape::line(vec![self.start.as_pos2(), self.end.as_pos2()], self.stroke)
    }

    pub fn to_screen(self, screen_params: ScreenParams) -> Self {
        Self {
            start: self.start.as_screen(screen_params),
            end: self.end.as_screen(screen_params),
            stroke: self.stroke,
        }
    }
}
