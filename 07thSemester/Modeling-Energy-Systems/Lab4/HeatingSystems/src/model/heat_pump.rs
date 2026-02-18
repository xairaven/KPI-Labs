#[derive(Default)]
pub struct HeatPump {
    pub electric_power: f64,
    pub temperature_source: f64,
    pub temperature_in: f64,
    pub temperature_out: f64,
}

impl HeatPump {
    fn coefficient_of_performance(&self) -> f64 {
        (self.temperature_out + 273.15) / (self.temperature_out - self.temperature_source)
    }

    pub fn heat_power(&self) -> f64 {
        self.electric_power * self.coefficient_of_performance()
    }

    pub fn flow_rate(&self) -> f64 {
        3600.0 * self.heat_power() / ((self.temperature_out - self.temperature_in) * 4.187)
    }
}
