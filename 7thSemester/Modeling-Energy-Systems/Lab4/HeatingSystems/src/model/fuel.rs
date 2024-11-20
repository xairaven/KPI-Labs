use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug)]
pub enum Fuel {
    NaturalGas,
    Coal,
    WoodPellets,
    Diesel,
}

impl Fuel {
    pub fn heating_value(&self) -> f64 {
        match self {
            Fuel::NaturalGas => 9.5,
            Fuel::Coal => 7.0,
            Fuel::WoodPellets => 4.2,
            Fuel::Diesel => 12.0,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Fuel::NaturalGas => String::from("Natural Gas"),
            Fuel::Coal => String::from("Coal"),
            Fuel::WoodPellets => String::from("Wood pellets"),
            Fuel::Diesel => String::from("Diesel fuel"),
        }
    }

    pub fn values() -> Vec<Self> {
        Self::iter().collect()
    }
}
