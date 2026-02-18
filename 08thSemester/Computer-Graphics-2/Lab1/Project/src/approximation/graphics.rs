use crate::geometry::line2d::Line2D;
use crate::geometry::point2d::Point2D;
use egui::Stroke;

pub fn linear_line(coefficients: (f32, f32), stroke: Stroke) -> Line2D {
    let (a, b) = coefficients;

    let x_start = -100.0;
    let x_end = 100.0;

    let start = Point2D::new(x_start, a + b * x_start);
    let end = Point2D::new(x_end, a + b * x_end);

    Line2D::new(start, end, stroke)
}

pub fn parabola_lines(coefficients: (f32, f32, f32), stroke: Stroke) -> Vec<Line2D> {
    let (a, b, c) = coefficients;

    let mut points: Vec<Point2D> = Vec::new();

    let x_start = -100.0;
    let x_end = 100.0;

    let mut x = x_start;
    while x <= x_end + 0.0001 {
        let point = Point2D::new(x, a + b * x + c * x.powi(2));
        points.push(point);

        x += 0.05;
    }

    points
        .windows(2)
        .map(|p| Line2D::new(p[0], p[1], stroke))
        .collect()
}
