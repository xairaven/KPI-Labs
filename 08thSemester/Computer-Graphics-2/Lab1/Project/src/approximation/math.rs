use crate::geometry::point2d::Point2D;
use nalgebra::{Matrix2, Matrix3, Vector2, Vector3};
use thiserror::Error;

pub fn linear(points: &[Point2D]) -> Result<(f32, f32), MathError> {
    let n = points.len() as f32;
    let sum_xs = points.iter().map(|p| p.x).sum::<f32>();
    let sum_xs_quadratic = points.iter().map(|p| p.x.powi(2)).sum::<f32>();

    let a = Matrix2::new(n, sum_xs, sum_xs, sum_xs_quadratic);

    let sum_ys = points.iter().map(|p| p.y).sum::<f32>();
    let sum_xys = points.iter().map(|p| p.x * p.y).sum::<f32>();
    let b = Vector2::new(sum_ys, sum_xys);

    match a.lu().solve(&b) {
        Some(matrix) => Ok((matrix[0], matrix[1])),
        None => Err(MathError::CannotSolveMatrix),
    }
}

pub fn quadratic(points: &[Point2D]) -> Result<(f32, f32, f32), MathError> {
    let n = points.len() as f32;
    let sum_xs = points.iter().map(|p| p.x).sum::<f32>();
    let sum_xs_quadratic = points.iter().map(|p| p.x.powi(2)).sum::<f32>();
    let sum_xs_cubic = points.iter().map(|p| p.x.powi(3)).sum::<f32>();
    let sum_xs_quartic = points.iter().map(|p| p.x.powi(4)).sum::<f32>();

    let a = Matrix3::new(
        n,
        sum_xs,
        sum_xs_quadratic,
        sum_xs,
        sum_xs_quadratic,
        sum_xs_cubic,
        sum_xs_quadratic,
        sum_xs_cubic,
        sum_xs_quartic,
    );

    let sum_ys = points.iter().map(|p| p.y).sum::<f32>();
    let sum_xys = points.iter().map(|p| p.x * p.y).sum::<f32>();
    let sum_xs_quadratic_ys = points.iter().map(|p| p.x.powi(2) * p.y).sum::<f32>();
    let b = Vector3::new(sum_ys, sum_xys, sum_xs_quadratic_ys);

    match a.lu().solve(&b) {
        Some(matrix) => Ok((matrix[0], matrix[1], matrix[2])),
        None => Err(MathError::CannotSolveMatrix),
    }
}

#[derive(Error, Debug)]
pub enum MathError {
    #[error("Cannot solve linear equations.")]
    CannotSolveMatrix,
}
