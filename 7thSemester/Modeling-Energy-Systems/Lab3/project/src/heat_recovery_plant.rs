use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct HeatRecoveryPlant {
    pub is_enabled: bool,

    pub title: String,
    pub wind_flow_part: f32,
    pub efficiency: f32,
}
