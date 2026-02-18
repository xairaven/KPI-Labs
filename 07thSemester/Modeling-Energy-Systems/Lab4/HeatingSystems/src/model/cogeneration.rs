pub struct CogenerationUnit {
    pub efficiency_electric: f64,
    pub efficiency_thermal: f64,
    pub temp_in: f64,
    pub fluid_flow_rate: f64,
    pub fuel_consumption: f64,
    pub heating_value: f64,
}

impl CogenerationUnit {
    pub fn electric_power(&self) -> f64 {
        self.fuel_consumption * self.heating_value * self.efficiency_electric / 100.0
    }

    pub fn thermal_power(&self) -> f64 {
        self.fuel_consumption * self.heating_value * self.efficiency_thermal / 100.0
    }

    pub fn temperature_out(&self) -> f64 {
        let electrical_power = self.electric_power();
        self.temp_in + 3600.0 * electrical_power / (4.187 * self.fluid_flow_rate)
    }
}
