use crate::geometry::point2d::Point2D;

pub enum Functions {
    First,
    Second,
}

impl Functions {
    pub fn name(&self) -> &'static str {
        match self {
            Functions::First => "x^3+x",
            Functions::Second => "√(1+x)",
        }
    }

    pub fn vec() -> Vec<Functions> {
        vec![Functions::First, Functions::Second]
    }

    pub fn points(&self) -> Vec<Point2D> {
        match self {
            Functions::First => {
                vec![
                    Point2D::new(-0.900, -1.629),
                    Point2D::new(-0.745, -1.160),
                    Point2D::new(-0.591, -0.797),
                    Point2D::new(-0.436, -0.519),
                    Point2D::new(-0.282, -0.304),
                    Point2D::new(-0.127, -0.129),
                    Point2D::new(0.027, 0.027),
                    Point2D::new(0.182, 0.188),
                    Point2D::new(0.336, 0.374),
                    Point2D::new(0.491, 0.609),
                    Point2D::new(0.645, 0.914),
                    Point2D::new(0.800, 1.312),
                ]
            },
            Functions::Second => {
                vec![
                    Point2D::new(-0.900, 0.316),
                    Point2D::new(-0.745, 0.505),
                    Point2D::new(-0.591, 0.640),
                    Point2D::new(-0.436, 0.751),
                    Point2D::new(-0.282, 0.847),
                    Point2D::new(-0.127, 0.934),
                    Point2D::new(0.027, 1.014),
                    Point2D::new(0.182, 1.087),
                    Point2D::new(0.336, 1.156),
                    Point2D::new(0.491, 1.221),
                    Point2D::new(0.645, 1.283),
                    Point2D::new(0.800, 1.342),
                ]
            },
        }
    }
}
