use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Building {
    pub apartment_area: f32,
    pub walls: Vec<Wall>,
    pub windows: Vec<Window>,
}

#[derive(Deserialize, Debug)]
pub struct Wall {
    pub width: f32,
    pub height: f32,

    pub layers: Vec<WallLayer>,

    pub absorption_coefficient: f32,
    pub emissivity_coefficient: f32,

    pub inner_surface_heat_transfer_coefficient: f32,
    pub outer_surface_heat_transfer_coefficient: f32,
}

#[derive(Deserialize, Debug)]
pub struct WallLayer {
    pub material: String,
    pub thickness: f32,
    pub thermal_conductivity: f32,
}

#[derive(Deserialize, Debug)]
pub struct Window {
    pub width: f32,
    pub height: f32,

    pub panes_amount: f32,

    pub glass_type: String,

    pub emissivity_coefficient: f32,
    pub heat_transfer_coefficient: f32,
    pub solar_transmittance_coefficient: f32,

    pub orientation: WindowOrientation,
}

#[derive(Deserialize, Debug)]
pub enum WindowOrientation {
    North,
    South,
    West,
    East,
}
