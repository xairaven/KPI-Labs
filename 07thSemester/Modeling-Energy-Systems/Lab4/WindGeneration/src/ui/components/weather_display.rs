use crate::api_state::ApiStatus;
use crate::config::Config;
use crate::context::Context;
use crate::ui::styles::colors;
use egui::{ComboBox, Grid, RichText};

pub fn show(config: &Config, context: &mut Context, ui: &mut egui::Ui) {
    Grid::new("CityGrid").num_columns(2).show(ui, |ui| {
        ui.label(RichText::new("Choose city:").color(colors::WHITE));

        ComboBox::from_label("")
            .selected_text(&context.local_data[context.selected_city_index].name) // Display the currently selected option.
            .show_ui(ui, |ui| {
                for (i, option) in context.local_data.iter().enumerate() {
                    ui.selectable_value(&mut context.selected_city_index, i, &option.name);
                }
            });
    });

    let new_chosen_city = &context.local_data[context.selected_city_index].name;
    if !new_chosen_city.eq(&context.api_state.city) {
        let _ = context.api_state.update(&config.api_key, new_chosen_city);
    }

    ui.add_space(10.0);

    ui.vertical_centered(|ui| {
        ui.heading("In-Moment Data:");
    });

    Grid::new("WeatherGrid").num_columns(2).show(ui, |ui| {
        if let Some(weather) = &context.api_state.weather {
            if let ApiStatus::Online = &context.api_state.status {
                ui.label("Latitude:");
                ui.label(format!("{:.2}", weather.location.latitude));
                ui.end_row();

                ui.label("Longitude:");
                ui.label(format!("{:.2}", weather.location.longitude));

                ui.end_row();

                ui.label("Weather:");
                ui.label(&weather.main_status);

                ui.end_row();

                ui.label("Temperature:");
                ui.label(format!("{:.2} °C", weather.temperature));

                ui.end_row();

                ui.label("Wind Speed:");
                ui.label(format!("{:.2} m/s", weather.wind_speed));

                ui.end_row();

                ui.label("Wind Direction:");
                ui.label(format!("{:.2}°", weather.wind_direction));

                ui.end_row();
            }
        }

        ui.label("Status:");
        match context.api_state.status {
            ApiStatus::Online => ui.label(RichText::new("Online").color(colors::GREEN)),
            ApiStatus::Offline => ui.label(RichText::new("Offline").color(colors::RED)),
        };
        ui.end_row();
        ui.end_row();

        ui.label(
            RichText::new("Last Update:")
                .color(colors::WHITE)
                .size(11.0),
        );
        ui.label(
            RichText::new(format!(
                "{} s. from now",
                context.api_state.last_updated.elapsed().as_secs()
            ))
            .size(11.0),
        );

        ui.end_row();

        ui.label(
            RichText::new("Refresh Interval:")
                .color(colors::WHITE)
                .size(11.0),
        );
        ui.label(
            RichText::new(format!(
                "{} s.",
                context.api_state.refresh_interval_seconds()
            ))
            .size(11.0),
        );
    });

    ui.add_space(10.0);

    ui.vertical_centered(|ui| {
        if ui.button("Update ♻").clicked() {
            let _ = context.api_state.update(&config.api_key, new_chosen_city);
        }
    });
}
