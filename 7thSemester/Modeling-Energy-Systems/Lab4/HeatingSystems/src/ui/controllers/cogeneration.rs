use crate::model::cogeneration::CogenerationUnit;
use crate::ui::controller_trait::MenuController;
use crate::ui::controllers::fuel::fuel_menu;
use std::io;

#[derive(Default)]
pub struct CogenerationController {}

impl MenuController for CogenerationController {
    fn view(&self) {
        let heating_value = fuel_menu();

        println!("Fuel Consumption (Units depending on chosen fuel) (5..20):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let fuel_consumption: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Fluid Flow Rate, kg/h (500..3000):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let fluid_flow_rate: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Water Temperature-In, °C:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let temp_in: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Electric Efficiency, % (35..44):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let efficiency_electric: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Thermal Efficiency, % (40..45):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let efficiency_thermal: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("-- RESULTS --");
        let unit = CogenerationUnit {
            efficiency_electric,
            efficiency_thermal,
            temp_in,
            fluid_flow_rate,
            fuel_consumption,
            heating_value,
        };

        let electric_power = unit.electric_power();
        let thermal_power = unit.thermal_power();
        let temp_out = unit.temperature_out();
        let delta = temp_out - temp_in;
        println!("Electrical Power, kW: {:.2}", electric_power);
        println!("Thermal Power, kW: {:.2}", thermal_power);
        println!("Temperature-Out, °C: {:.2}", temp_out);
        println!("ΔT, °C: {:.2}", delta);
        println!("\n");
    }

    fn name(&self) -> String {
        String::from("Cogeneration Unit")
    }
}
