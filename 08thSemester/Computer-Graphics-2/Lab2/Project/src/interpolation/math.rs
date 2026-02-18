use crate::geometry::point2d::Point2D;

pub fn linear(points: &[Point2D]) -> Vec<Point2D> {
    let mut result: Vec<Point2D> = Vec::with_capacity(points.len() + 1);

    if let Some(first_point) = points.first() {
        result.push(*first_point);
    }

    points.windows(2).for_each(|pair| {
        let x = (pair[1].x + pair[0].x) / 2.0;
        let y = pair[0].y
            + (x - pair[0].x) / (pair[1].x - pair[0].x) * (pair[1].y - pair[0].y);
        result.push(Point2D::new(x, y));
    });

    if let Some(last_point) = points.last() {
        result.push(*last_point);
    }

    result
}

pub fn lagrange(points: &[Point2D]) -> Vec<Point2D> {
    let mut result = Vec::new();
    let n = points.len();

    if n < 2 {
        return result;
    }

    let min_x = points.first().unwrap().x;
    let max_x = points.last().unwrap().x;

    let num_points = 100;
    let step = (max_x - min_x) / num_points as f32;

    for i in 0..num_points {
        let x = min_x + step * i as f32;
        let mut y = 0.0;

        for i in 0..n {
            let (x_i, y_i) = (points[i].x, points[i].y);

            let mut L = 1.0;
            for j in 0..n {
                if i != j {
                    let (x_j, _) = (points[j].x, points[j].y);
                    L *= (x - x_j) / (x_i - x_j);
                }
            }

            y += y_i * L;
        }

        result.push(Point2D::new(x, y));
    }

    if let Some(last_point) = points.last() {
        result.push(*last_point);
    }

    result
}
