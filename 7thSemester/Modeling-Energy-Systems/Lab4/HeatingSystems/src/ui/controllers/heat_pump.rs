use crate::model::heat_pump::HeatPump;
use crate::ui::controller_trait::MenuController;
use std::io;

#[derive(Default)]
pub struct HeatPumpController {}

impl MenuController for HeatPumpController {
    fn view(&self) {
        println!("Electric Power, kW:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let electric_power: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Source Temperature, °C:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let temp_source: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Water Temperature-In, °C (30..40):");
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

        println!("Water Temperature-Out, °C (50..65):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let temp_out: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("-- RESULTS --");
        let pump = HeatPump {
            electric_power,
            temperature_source: temp_source,
            temperature_in: temp_in,
            temperature_out: temp_out,
        };

        let heat_power = pump.heat_power();
        let flow_rate = pump.flow_rate();
        println!("Heat Power, kW: {:.2}", heat_power);
        println!("Fluid Flow Rate, kg/h: {:.2}", flow_rate);
        println!("\n");
    }

    fn name(&self) -> String {
        String::from("Heat Pump")
    }
}
