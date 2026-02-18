use crate::ui::controller_trait::MenuController;
use crate::ui::controllers::boiler::BoilerController;
use crate::ui::controllers::cogeneration::CogenerationController;
use crate::ui::controllers::exit::ExitController;
use crate::ui::controllers::heat_exchanger::HeatExchangerController;
use crate::ui::controllers::heat_pump::HeatPumpController;
use std::io;

pub fn menu() {
    let controllers: Vec<Box<dyn MenuController>> = vec![
        Box::new(BoilerController::default()),
        Box::new(CogenerationController::default()),
        Box::new(HeatPumpController::default()),
        Box::new(HeatExchangerController::default()),
        Box::new(ExitController::default()),
    ];

    loop {
        println!("Choose a system:");
        for (i, item) in controllers.iter().enumerate() {
            println!("{}. {}", i + 1, item.name());
        }
        println!();

        let mut choice = String::new();
        if let Err(err) = io::stdin().read_line(&mut choice) {
            println!("Error: {}\n", err);
            continue;
        }
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Error: {}\n", err);
                continue;
            },
        };

        if choice < 1 || choice > controllers.len() as u32 {
            println!("Error: There's no variant like this\n");
            continue;
        }

        controllers[(choice - 1) as usize].view();
    }
}
