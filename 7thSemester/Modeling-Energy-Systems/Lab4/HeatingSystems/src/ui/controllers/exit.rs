use crate::ui::controller_trait::MenuController;

#[derive(Default)]
pub struct ExitController {}

impl MenuController for ExitController {
    fn view(&self) {
        println!("\nBye!");

        std::process::exit(0);
    }

    fn name(&self) -> String {
        String::from("Exit")
    }
}
