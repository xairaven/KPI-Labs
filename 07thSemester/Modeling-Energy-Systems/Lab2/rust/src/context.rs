use crate::errors::parse::ParseError;
use crate::math::angle::Angle;
use chrono::{DateTime, FixedOffset};

pub const DEFAULT_YEAR: u16 = 2024;

#[derive(Default)]
pub struct Context {
    pub is_full_day: bool,
    pub albedo: f64,
    pub datetime: DateTime<FixedOffset>,
    pub latitude: Angle,
    pub longitude: Angle,
    pub tilt_angle: Angle,
    pub azimuth: Angle,
    pub solar_irradiance: f64,
}

impl Context {
    pub fn parse_day_month(&mut self, day: &str, month: &str) -> Result<(), ParseError> {
        let datetime_str = format!("{DEFAULT_YEAR}-{month}-{day}T00:00:00+02:00");

        self.datetime = datetime_str
            .parse()
            .map_err(|_| ParseError::DateParse(String::default()))?;

        Ok(())
    }

    pub fn parse_datetime(
        &mut self, day: &str, month: &str, time: &str, utc: &str,
    ) -> Result<(), ParseError> {
        let datetime_str = format!("{DEFAULT_YEAR}-{month}-{day}T{time}:00{utc}");

        self.datetime = datetime_str
            .parse()
            .map_err(|_| ParseError::DateParse(String::default()))?;

        Ok(())
    }
}
