use crate::geometry::point2d::Point2D;

pub enum Functions {
    First,
    Second,
}

impl Functions {
    pub fn name(&self) -> &'static str {
        match self {
            Functions::First => "5x^5 − 3x^3 + x",
            Functions::Second => "3 * cos(2x) - 2 * sin(5x)",
        }
    }

    pub fn vec() -> Vec<Functions> {
        vec![Functions::First, Functions::Second]
    }

    pub fn points(&self) -> Vec<Point2D> {
        match self {
            Functions::First => {
                vec![
                    Point2D::new(-0.900, -1.66545),
                    Point2D::new(-0.745, -0.652015621),
                    Point2D::new(-0.591, -0.332226563),
                    Point2D::new(-0.436, -0.266131978),
                    Point2D::new(-0.282, -0.22363963),
                    Point2D::new(-0.127, -0.121020043),
                    Point2D::new(0.027, 0.026941023),
                    Point2D::new(0.182, 0.164912747),
                    Point2D::new(0.336, 0.243613283),
                    Point2D::new(0.491, 0.278572406),
                    Point2D::new(0.645, 0.398164307),
                    Point2D::new(0.800, 0.9024),
                ]
            },
            Functions::Second => {
                vec![
                    Point2D::new(-0.900, -2.636666519),
                    Point2D::new(-0.745, -0.859616391),
                    Point2D::new(-0.591, 1.508248108),
                    Point2D::new(-0.436, 3.570097697),
                    Point2D::new(-0.282, 4.509570984),
                    Point2D::new(-0.127, 4.0901002),
                    Point2D::new(0.027, 2.726446441),
                    Point2D::new(0.182, 1.224433251),
                    Point2D::new(0.336, 0.359647977),
                    Point2D::new(0.491, 0.398270138),
                    Point2D::new(0.645, 0.99798397),
                    Point2D::new(0.800, 1.426006424),
                ]
            },
        }
    }
}
