use crate::model::boiler::Boiler;
use crate::ui::controller_trait::MenuController;
use crate::ui::controllers::fuel::fuel_menu;
use std::io;

#[derive(Default)]
pub struct BoilerController {}

impl MenuController for BoilerController {
    fn view(&self) {
        let heating_value = fuel_menu();

        println!("Boiler Efficiency, % (80..92):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let efficiency: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

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

        println!("-- RESULTS --");
        let boiler = Boiler {
            fuel_consumption,
            heating_value,
            efficiency,
            temp_in,
            fluid_flow_rate,
        };

        let power = boiler.power();
        let temp_out = boiler.temperature_out();
        let delta = temp_out - temp_in;
        println!("Boiler Power, kW: {:.2}", power);
        println!("Temperature-Out, °C: {:.2}", temp_out);
        println!("ΔT, °C: {:.2}", delta);
        println!("\n");
    }

    fn name(&self) -> String {
        String::from("Boiler")
    }
}
