use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub city: String,

    pub avg_temperature_per_month: Vec<f32>,
    pub solar_radiation_per_month: Vec<f32>,

    pub outdoor_temperature: Vec<Vec<f32>>,
}
