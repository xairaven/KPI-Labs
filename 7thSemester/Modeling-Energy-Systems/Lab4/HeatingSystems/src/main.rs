fn main() {
    ui::core::menu();
}

pub mod model {
    pub mod boiler;
    pub mod cogeneration;
    pub mod fuel;
    pub mod heat_exchanger;
    pub mod heat_pump;
    pub mod system_types;
}
pub mod ui {
    pub mod core;
    pub mod controllers {
        pub mod boiler;
        pub mod cogeneration;
        pub mod heat_exchanger;
        pub mod heat_pump;

        pub mod fuel;

        pub mod exit;
    }
    pub mod controller_trait;
}
