use crate::model::Mode;

pub mod building;
pub mod context;
pub mod date;
pub mod error;
pub mod heat_recovery_plant;
pub mod json;
pub mod location;
pub mod model;

pub const JSON_FILE_NAME: &str = "data.json";

fn main() {
    let context = match json::parse(JSON_FILE_NAME) {
        Ok(value) => value,
        Err(err) => {
            println!("Error: {}", err);
            std::process::exit(1);
        },
    };

    for (index, month) in date::MONTHS.iter().enumerate() {
        println!("MONTH: {month}");

        let total_energy_demand_heating =
            model::energy_demand(Mode::Heating, (index + 1) as u32, &context);
        let total_energy_demand_cooling =
            model::energy_demand(Mode::Cooling, (index + 1) as u32, &context);

        let diff = total_energy_demand_heating - total_energy_demand_cooling;
        let mode = if diff >= 0.0 { "Heating" } else { "Cooling" };

        println!("- Total Energy Demand | {}: {}", mode, f32::abs(diff));

        println!();
    }
}
