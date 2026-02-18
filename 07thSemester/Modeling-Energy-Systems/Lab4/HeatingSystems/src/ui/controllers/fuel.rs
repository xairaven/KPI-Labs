use crate::model::fuel::Fuel;
use std::io;

pub fn fuel_menu() -> f64 {
    let variants = Fuel::values();

    loop {
        println!("Choose fuel:");
        for (i, item) in variants.iter().enumerate() {
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

        if choice < 1 || choice > variants.len() as u32 {
            println!("Error: There's no variant like this\n");
            continue;
        }

        return variants[choice as usize - 1].heating_value();
    }
}
