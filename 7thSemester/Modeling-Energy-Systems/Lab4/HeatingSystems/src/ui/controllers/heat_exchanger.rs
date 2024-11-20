use crate::model::heat_exchanger::HeatExchanger;
use crate::ui::controller_trait::MenuController;
use std::io;

#[derive(Default)]
pub struct HeatExchangerController {}

impl MenuController for HeatExchangerController {
    fn view(&self) {
        println!("Efficiency, % (85..90):");
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

        println!("Indoor Temperature, °C:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let temp_indoor: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Outdoor Temperature, °C:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let temp_outdoor: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Number of People in a house:");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let number_of_people: usize = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("Normative air exchange per person, m³/h (12..20):");
        let mut value_buffer = String::new();
        if let Err(err) = io::stdin().read_line(&mut value_buffer) {
            println!("Error: {}\n", err);
            return;
        }
        let norm_air_exchange: f64 = match value_buffer.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                return;
            },
        };

        println!("-- RESULTS --");
        let exchanger = HeatExchanger {
            efficiency,
            temp_indoor,
            temp_outdoor,
            number_of_people,
            norm_air_exchange,
        };

        let air_flow_volume = exchanger.air_flow_volume();
        let thermal_loss = exchanger.thermal_loss();
        let recuperator_heat = exchanger.heat();
        let heating_energy = exchanger.heating_energy();
        let exhaust_temp_out = exchanger.exhaust_temp_out();
        let supplied_temp_out = exchanger.supplied_temp_out();

        println!("Total air flow volume, m³/h: {:.2}", air_flow_volume);
        println!("Thermal energy loss, kW: {:.2}", thermal_loss);
        println!("Recuperator heat flow, kW: {:.2}", recuperator_heat);
        println!("Heat required for air heating, kW: {:.2}", heating_energy);
        println!(
            "Exhaust air temperature (after rec.), °C: {:.2}",
            exhaust_temp_out
        );
        println!(
            "Supplied air temperature (after rec.), °C: {:.2}",
            supplied_temp_out
        );
        println!("\n");
    }

    fn name(&self) -> String {
        String::from("Recuperative Heat Exchanger")
    }
}
