use crate::graphics::Viewport;
use crate::graphics::primitives::line::Line;
use crate::graphics::primitives::point::Point;
use crate::graphics::units::Centimeter;
use eframe::epaint::Stroke;

pub mod defaults {
    use crate::graphics::primitives::point::Point;
    use crate::graphics::units::Centimeter;
    use egui::{Color32, Stroke};

    pub const ORIGIN: Point = Point {
        x: Centimeter(0.0),
        y: Centimeter(0.0),
    };
    pub const UNIT: Centimeter = Centimeter(1.0);

    pub const AXIS_RED: Stroke = Stroke {
        width: 2.0,
        color: Color32::RED,
    };
    pub const AXIS_GREEN: Stroke = Stroke {
        width: 2.0,
        color: Color32::GREEN,
    };
    pub const GRID_GRAY: Stroke = Stroke {
        width: 0.8,
        color: Color32::GRAY,
    };
}

pub struct Grid2DBuilder {
    pub origin: Point,
    pub unit: Centimeter,

    pub bounds_x: (Option<Centimeter>, Option<Centimeter>),
    pub bounds_y: (Option<Centimeter>, Option<Centimeter>),

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Default for Grid2DBuilder {
    fn default() -> Self {
        Self {
            origin: defaults::ORIGIN,
            unit: defaults::UNIT,

            bounds_x: (None, None),
            bounds_y: (None, None),

            axes_strokes: (defaults::AXIS_RED, defaults::AXIS_GREEN),
            grid_stroke: defaults::GRID_GRAY,
        }
    }
}

#[allow(dead_code)]
impl Grid2DBuilder {
    pub fn with_origin(mut self, origin: Point) -> Self {
        self.origin = origin;
        self
    }

    pub fn with_unit(mut self, unit: Centimeter) -> Self {
        self.unit = unit;
        self
    }

    pub fn with_bounds_x(
        mut self, min: Option<Centimeter>, max: Option<Centimeter>,
    ) -> Self {
        self.bounds_x = (min, max);
        self
    }

    pub fn with_bounds_y(
        mut self, min: Option<Centimeter>, max: Option<Centimeter>,
    ) -> Self {
        self.bounds_y = (min, max);
        self
    }

    pub fn with_axes_strokes(mut self, stroke_x: Stroke, stroke_y: Stroke) -> Self {
        self.axes_strokes = (stroke_x, stroke_y);
        self
    }

    pub fn with_grid_stroke(mut self, stroke: Stroke) -> Self {
        self.grid_stroke = stroke;
        self
    }

    pub fn build(self) -> Grid2D {
        Grid2D {
            is_enabled: true,
            are_axes_enabled: false,
            origin: self.origin,
            unit: self.unit,
            bounds: GridBounds {
                x: (self.bounds_x.0, self.bounds_x.1),
                y: (self.bounds_y.0, self.bounds_y.1),
            },
            axes_strokes: self.axes_strokes,
            grid_stroke: self.grid_stroke,
        }
    }
}

#[derive(Debug)]
pub struct GridBounds {
    pub x: (Option<Centimeter>, Option<Centimeter>),
    pub y: (Option<Centimeter>, Option<Centimeter>),
}

impl GridBounds {
    pub fn view_bounds(&self, viewport: &Viewport) -> ViewGridBounds {
        // Canvas bounds in centimeters
        let v = viewport.state.bounds.to_centimeters(viewport);

        ViewGridBounds {
            minimum_x: self
                .x
                .0
                .map_or(v.minimum_x, |c| Centimeter(c.max(v.minimum_x.value()))),
            maximum_x: self
                .x
                .1
                .map_or(v.maximum_x, |c| Centimeter(c.min(v.maximum_x.value()))),
            minimum_y: self
                .y
                .0
                .map_or(v.minimum_y, |c| Centimeter(c.max(v.minimum_y.value()))),
            maximum_y: self
                .y
                .1
                .map_or(v.maximum_y, |c| Centimeter(c.min(v.maximum_y.value()))),
        }
    }
}

pub struct ViewGridBounds {
    pub minimum_x: Centimeter,
    pub maximum_x: Centimeter,
    pub minimum_y: Centimeter,
    pub maximum_y: Centimeter,
}

#[derive(Debug)]
pub struct Grid2D {
    pub is_enabled: bool,
    pub are_axes_enabled: bool,

    pub origin: Point,
    pub unit: Centimeter,

    pub bounds: GridBounds,

    pub axes_strokes: (Stroke, Stroke),
    pub grid_stroke: Stroke,
}

impl Grid2D {
    pub fn lines(&self, viewport: &Viewport) -> Vec<Line<Point>> {
        let mut lines = vec![];

        // Minimum and maximum bounds in centimeters for the viewport, clamped by the grid bounds
        let view_bounds = self.bounds.view_bounds(viewport);

        if self.is_enabled {
            lines.extend(self.grid(&view_bounds));
        }
        if self.are_axes_enabled {
            lines.extend(self.axes(&view_bounds));
        }

        lines
    }

    fn grid(&self, bounds: &ViewGridBounds) -> Vec<Line<Point>> {
        let mut lines = vec![];

        for x in bounds.minimum_x.value() as i32..=bounds.maximum_x.value() as i32 {
            if self.are_axes_enabled && x == self.origin.x.value() as i32 {
                continue;
            }

            if x % self.unit.value() as i32 == 0 {
                let line = Line {
                    start: Point {
                        x: Centimeter(x as f64),
                        y: bounds.minimum_y,
                    },
                    end: Point {
                        x: Centimeter(x as f64),
                        y: bounds.maximum_y,
                    },
                    stroke: self.grid_stroke,
                };
                lines.push(line);
            }
        }
        for y in bounds.minimum_y.value() as i32..=bounds.maximum_y.value() as i32 {
            if self.are_axes_enabled && y == self.origin.y.value() as i32 {
                continue;
            }

            if y % self.unit.value() as i32 == 0 {
                let line = Line {
                    start: Point {
                        x: bounds.minimum_x,
                        y: Centimeter(y as f64),
                    },
                    end: Point {
                        x: bounds.maximum_x,
                        y: Centimeter(y as f64),
                    },
                    stroke: self.grid_stroke,
                };
                lines.push(line);
            }
        }

        lines
    }

    fn axes(&self, bounds: &ViewGridBounds) -> Vec<Line<Point>> {
        let mut lines = vec![];

        // Axes
        let axis_x = Line {
            start: Point {
                x: bounds.minimum_x,
                y: self.origin.y,
            },
            end: Point {
                x: bounds.maximum_x,
                y: self.origin.y,
            },
            stroke: self.axes_strokes.0,
        };
        let axis_y = Line {
            start: Point {
                x: self.origin.x,
                y: bounds.minimum_y,
            },
            end: Point {
                x: self.origin.x,
                y: bounds.maximum_y,
            },
            stroke: self.axes_strokes.1,
        };

        lines.push(axis_x);
        lines.push(axis_y);

        lines
    }
}
