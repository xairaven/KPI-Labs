pub enum WindGeneratorType {
    HorizontalAxis,
    VerticalAxis,
}

impl WindGeneratorType {
    pub fn index(&self) -> usize {
        match self {
            WindGeneratorType::HorizontalAxis => 1,
            WindGeneratorType::VerticalAxis => 2,
        }
    }

    pub fn by_index(index: usize) -> Self {
        if index == 1 {
            WindGeneratorType::HorizontalAxis
        } else {
            WindGeneratorType::VerticalAxis
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            WindGeneratorType::HorizontalAxis => String::from("Horizontal-Axis"),
            WindGeneratorType::VerticalAxis => String::from("Vertical-Axis"),
        }
    }
}

pub trait WindGenerator {
    fn rotor_area(&self) -> f32;
    fn wind_energy_efficiency_coefficient(&self) -> f32;

    fn air_density(&self, ambient_temperature: f32) -> f32 {
        const GAS_PRESSURE: f32 = 101.325;
        const UNIVERSAL_GAS_CONSTANT: f32 = 8.314;
        const MOLAR_MASS: f32 = 28.98;

        (GAS_PRESSURE * MOLAR_MASS) / (UNIVERSAL_GAS_CONSTANT * (ambient_temperature + 273.15))
    }

    fn power(&self, ambient_temperature: f32, wind_speed: f32) -> f32 {
        let air_density = self.air_density(ambient_temperature);

        0.5 * air_density * self.rotor_area() * wind_speed.powf(3.0)
    }

    fn electrical_power(&self, ambient_temperature: f32, wind_speed: f32) -> f32 {
        self.power(ambient_temperature, wind_speed) * self.wind_energy_efficiency_coefficient()
    }
}

pub struct VerticalWindGenerator {
    pub blade_diameter: f32,
    pub blade_height: f32,
    pub wind_energy_efficiency_coefficient: f32,
}

impl VerticalWindGenerator {
    pub fn new(blade_diameter: f32, blade_height: f32) -> Self {
        Self {
            blade_diameter,
            blade_height,
            wind_energy_efficiency_coefficient: 0.3,
        }
    }
}

impl WindGenerator for VerticalWindGenerator {
    fn rotor_area(&self) -> f32 {
        self.blade_diameter * self.blade_height
    }

    fn wind_energy_efficiency_coefficient(&self) -> f32 {
        self.wind_energy_efficiency_coefficient
    }
}

pub struct HorizontalWindGenerator {
    pub blade_diameter: f32,
    pub wind_energy_efficiency_coefficient: f32,
}

impl HorizontalWindGenerator {
    pub fn new(blade_diameter: f32) -> Self {
        Self {
            blade_diameter,
            wind_energy_efficiency_coefficient: 0.4,
        }
    }
}

impl WindGenerator for HorizontalWindGenerator {
    fn rotor_area(&self) -> f32 {
        (std::f32::consts::PI * self.blade_diameter.powf(2.0)) / 4.0
    }

    fn wind_energy_efficiency_coefficient(&self) -> f32 {
        self.wind_energy_efficiency_coefficient
    }
}
