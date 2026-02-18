pub struct Boiler {
    pub fuel_consumption: f64,
    pub heating_value: f64,
    pub efficiency: f64,
    pub temp_in: f64,
    pub fluid_flow_rate: f64,
}

impl Boiler {
    pub fn power(&self) -> f64 {
        self.fuel_consumption * self.heating_value * self.efficiency / 100.0
    }

    pub fn temperature_out(&self) -> f64 {
        let boiler_power = self.power();
        self.temp_in + 3600.0 * boiler_power / (4.187 * self.fluid_flow_rate)
    }
}
