#[derive(Default)]
pub struct HeatExchanger {
    pub efficiency: f64,
    pub temp_indoor: f64,
    pub temp_outdoor: f64,
    pub number_of_people: usize,
    pub norm_air_exchange: f64,
}

impl HeatExchanger {
    pub fn air_flow_volume(&self) -> f64 {
        self.number_of_people as f64 * self.norm_air_exchange
    }

    pub fn thermal_loss(&self) -> f64 {
        1.0 * (self.temp_indoor - self.temp_outdoor) * self.air_flow_volume() / 3600.0
    }

    pub fn heat(&self) -> f64 {
        self.thermal_loss() * self.efficiency / 100.0
    }

    pub fn heating_energy(&self) -> f64 {
        self.thermal_loss() - self.heat()
    }

    pub fn exhaust_temp_out(&self) -> f64 {
        self.temp_outdoor + 3600.0 * self.heating_energy() / self.air_flow_volume()
    }

    pub fn supplied_temp_out(&self) -> f64 {
        self.temp_indoor + 3600.0 * self.heating_energy() / self.air_flow_volume()
    }
}
