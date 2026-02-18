use crate::api_state::ApiState;
use crate::services::local_data::CityLocalData;

pub struct Context {
    pub api_state: ApiState,

    pub local_data: Vec<CityLocalData>,
    pub selected_city_index: usize,

    pub generator_type_index: usize,
    pub blade_diameter: f32,
    pub blade_height: f32,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            api_state: Default::default(),

            local_data: Default::default(),

            generator_type_index: 1,
            blade_diameter: 0.0,
            blade_height: 0.0,

            selected_city_index: 0,
        }
    }
}
