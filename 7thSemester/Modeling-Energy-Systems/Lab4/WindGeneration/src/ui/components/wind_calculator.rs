use crate::api_state::ApiStatus;
use crate::context::Context;
use crate::model::wind_generator::{
    HorizontalWindGenerator, VerticalWindGenerator, WindGenerator, WindGeneratorType,
};
use crate::ui::styles::colors;
use crate::utils;
use egui::{ComboBox, DragValue, Grid, RichText};

pub fn show(context: &mut Context, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Wind Generator Power Calculator");
    });

    if let ApiStatus::Offline = context.api_state.status {
        ui.add_space(10.0);

        ui.label(
            RichText::new("Can't calculate in-moment data when API status is offline.")
                .color(colors::RED),
        );
        ui.end_row();
    }

    ui.add_space(10.0);

    Grid::new("CalculatorGrid").num_columns(2).show(ui, |ui| {
        ui.label(RichText::new("Choose Generator Type:").color(colors::WHITE));

        ComboBox::from_label("")
            .selected_text(WindGeneratorType::by_index(context.generator_type_index).to_string())
            .show_ui(ui, |ui| {
                ui.selectable_value(
                    &mut context.generator_type_index,
                    1,
                    WindGeneratorType::by_index(1).to_string(),
                );
                ui.selectable_value(
                    &mut context.generator_type_index,
                    2,
                    WindGeneratorType::by_index(2).to_string(),
                );
            });

        ui.end_row();

        let wind_generator_type = WindGeneratorType::by_index(context.generator_type_index);

        ui.label("Blade Diameter, m.");
        ui.add(
            DragValue::new(&mut context.blade_diameter)
                .speed(1)
                .range(0..=u64::MAX),
        );
        ui.end_row();

        if let WindGeneratorType::VerticalAxis = wind_generator_type {
            ui.label("Blade Height, m.");
            ui.add(
                DragValue::new(&mut context.blade_height)
                    .speed(1)
                    .range(0..=u64::MAX),
            );
            ui.end_row();
        }

        let generator: Box<dyn WindGenerator> = match wind_generator_type {
            WindGeneratorType::HorizontalAxis => {
                Box::new(HorizontalWindGenerator::new(context.blade_diameter))
            },
            WindGeneratorType::VerticalAxis => Box::new(VerticalWindGenerator::new(
                context.blade_diameter,
                context.blade_height,
            )),
        };

        ui.label("Rotor Area (m²):");
        ui.label(format!("{:.1}", generator.rotor_area()));
        ui.end_row();

        if let Some(weather) = &context.api_state.weather {
            if let ApiStatus::Online = context.api_state.status {
                ui.label("Air Density (kg/m³):");
                ui.label(format!("{:.1}", generator.air_density(weather.temperature)));
                ui.end_row();

                ui.label("Electrical Power (kW):");
                ui.label(format!(
                    "{:.2}",
                    generator.electrical_power(weather.temperature, weather.wind_speed) / 1000.0
                ));
                ui.end_row();
            }
        }

        ui.separator();
        ui.separator();
        ui.end_row();

        let city_data = &context.local_data[context.selected_city_index];
        let mut yearly_sum_power = 0.0;
        for month in 1..=12 {
            let temperature = city_data.average_temperatures[month - 1];
            let wind_speed = city_data.average_wind_speeds[month - 1];

            ui.label(format!("{}:", utils::date::MONTHS[month - 1]));
            let power = generator.electrical_power(temperature as f32, wind_speed as f32) / 1000.0
                * utils::date::DAYS_IN_MONTH[month - 1] as f32
                * 24.0
                * 60.0
                * 60.0;
            yearly_sum_power += power;
            ui.label(format!("{:.2}", power));
            ui.end_row();
        }

        ui.separator();
        ui.separator();

        ui.end_row();

        ui.label("Yearly:");
        ui.label(format!("{:.2}", yearly_sum_power));
        ui.end_row();
    });
}
