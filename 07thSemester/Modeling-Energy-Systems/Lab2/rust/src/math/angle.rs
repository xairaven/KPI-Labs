#[derive(Default)]
pub struct Angle {
    degree: f64,
    radian: f64,
}

impl Angle {
    pub fn from_degree(degree: f64) -> Self {
        Self {
            degree,
            radian: degree * std::f64::consts::PI / 180.0,
        }
    }

    pub fn from_radian(radian: f64) -> Self {
        Self {
            degree: radian * 180.0 / std::f64::consts::PI,
            radian,
        }
    }

    pub fn degree(&self) -> f64 {
        self.degree
    }

    pub fn radian(&self) -> f64 {
        self.radian
    }
}
