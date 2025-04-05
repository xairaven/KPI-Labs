use crate::geometry::point2d::Point2D;
use crate::interpolation;

#[derive(Default)]
pub struct InterpolationContext {
    pub points_view: Vec<Point2D>,

    is_initialized: bool,
    points: Vec<Point2D>,

    linear_points: Vec<Point2D>,
    lagrange_points: Vec<Point2D>,
}

impl InterpolationContext {
    pub fn initialize(&mut self) {
        self.points.clear();
        for point in &self.points_view {
            self.points.push(*point);
        }

        self.linear_points = interpolation::math::linear(&self.points);
        self.lagrange_points = interpolation::math::lagrange(&self.points);

        self.is_initialized = true;
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn points(&self) -> &[Point2D] {
        self.points.as_ref()
    }

    pub fn linear_points(&self) -> &[Point2D] {
        &self.linear_points
    }

    pub fn lagrange_points(&self) -> &[Point2D] {
        &self.lagrange_points
    }
}
