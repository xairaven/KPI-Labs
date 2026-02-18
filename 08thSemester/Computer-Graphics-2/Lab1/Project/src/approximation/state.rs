use crate::approximation;
use crate::approximation::math::MathError;
use crate::geometry::point2d::Point2D;

#[derive(Default)]
pub struct ApproximationState {
    pub points_view: Vec<Point2D>,

    is_initialized: bool,
    points: Vec<Point2D>,

    linear_coefficients: (f32, f32),
    quadratic_coefficients: (f32, f32, f32),
}

impl ApproximationState {
    pub fn initialize(&mut self) -> Result<(), MathError> {
        self.points.clear();
        for point in &self.points_view {
            self.points.push(*point);
        }

        self.linear_coefficients = approximation::math::linear(&self.points)?;
        self.quadratic_coefficients = approximation::math::quadratic(&self.points)?;

        self.is_initialized = true;

        Ok(())
    }

    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn points(&self) -> &Vec<Point2D> {
        self.points.as_ref()
    }

    pub fn linear_coefficients(&self) -> (f32, f32) {
        self.linear_coefficients
    }

    pub fn quadratic_coefficients(&self) -> (f32, f32, f32) {
        self.quadratic_coefficients
    }
}
