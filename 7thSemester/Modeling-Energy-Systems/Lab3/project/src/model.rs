use crate::building::Wall;
use crate::context::Context;
use crate::date;

const EFFICIENCY_FACTOR: f32 = 1.0; // Nu-gn

// Qnd
pub fn energy_demand(mode: Mode, month: u32, context: &Context) -> f32 {
    match mode {
        Mode::Heating => {
            let total_heat_gains = total_heat_gains(month, context);
            let vent_demand = vent_pre_heat_demand(month, context);

            // Qht
            let total_heat_transfer =
                transmission_heat_transfer(&mode, month, context) + vent_demand;

            total_heat_transfer - EFFICIENCY_FACTOR * total_heat_gains - vent_demand
        },
        Mode::Cooling => {
            let total_heat_gains = total_heat_gains(month, context);
            let vent_demand = vent_pre_cool_demand(month, context);

            // Qht
            let total_heat_transfer =
                transmission_heat_transfer(&mode, month, context) + vent_demand;

            total_heat_transfer - EFFICIENCY_FACTOR * total_heat_gains - vent_demand
        },
    }
}

// Qgn
fn total_heat_gains(month: u32, context: &Context) -> f32 {
    internal_heat_gains(month, context) * solar_heat_gains(month, context)
}

// Qint
fn internal_heat_gains(month: u32, context: &Context) -> f32 {
    let zone = context.building.apartment_area / 3.0;
    let sum = zone * 1.8 + zone * 2.0 + zone * 2.0;

    0.667 * date::DAYS_IN_MONTHS[(month - 1) as usize] as f32 * sum
}

// Qsol
fn solar_heat_gains(month: u32, context: &Context) -> f32 {
    let hours = date::HOURS_IN_MONTHS[month as usize - 1];
    let mut sum = 0.0;

    for wall in &context.building.walls {
        let insolation = 0.043
            * wall.absorption_coefficient
            * wall.emissivity_coefficient
            * wall.width
            * wall.height;
        let radiation = context.location.solar_radiation_per_month[month as usize - 1];
        let form_coefficient = 0.5;
        let additional_heat_flow = 2.365
            * wall.emissivity_coefficient
            * wall.absorption_coefficient
            * wall.width
            * wall.height;

        let avg_heat_flow = insolation * radiation - form_coefficient * additional_heat_flow;
        sum += avg_heat_flow;
    }

    for window in &context.building.windows {
        let insolation =
            0.63 * window.solar_transmittance_coefficient * window.width * window.height;
        let radiation = context.location.solar_radiation_per_month[month as usize - 1];

        let avg_heat_flow = insolation * radiation;
        sum += avg_heat_flow;
    }

    hours as f32 * sum
}

// Qve,pre–heat
fn vent_pre_heat_demand(month: u32, context: &Context) -> f32 {
    let n = date::DAYS_IN_MONTHS[month as usize - 1];
    let heat_vent_transmission = heat_vent_transmission(context);

    let mut sum = 0.0;
    for i in 1..=24 {
        let system_working = 1.0;
        let outdoor_temp = context.location.outdoor_temperature[i - 1][month as usize - 1];

        sum +=
            system_working * heat_vent_transmission * (context.indoor_temp_heating - outdoor_temp);
    }

    n as f32 * sum
}

// Qve,pre–cool
fn vent_pre_cool_demand(month: u32, context: &Context) -> f32 {
    let n = date::DAYS_IN_MONTHS[month as usize - 1];
    let heat_vent_transmission = heat_vent_transmission(context);

    let mut sum = 0.0;
    for i in 1..=24 {
        let system_working = 1.0;
        let outdoor_temp = context.location.outdoor_temperature[i - 1][month as usize - 1];

        sum +=
            system_working * heat_vent_transmission * (outdoor_temp - context.indoor_temp_cooling);
    }

    n as f32 * sum
}

fn heat_vent_transmission(context: &Context) -> f32 {
    let temperature_correction_factor = 1.0
        - context.heat_recovery_plant.wind_flow_part * context.heat_recovery_plant.wind_flow_part;

    let air_waste = 112.0 / 168.0 * 0.6;

    0.336 * air_waste * temperature_correction_factor
}

// Qtr
fn transmission_heat_transfer(mode: &Mode, month: u32, context: &Context) -> f32 {
    let overall_transmission_coefficient = overall_transmission_coefficient(context);
    let hours_in_month = date::HOURS_IN_MONTHS[month as usize - 1];
    let avg_month_temperature = context.location.avg_temperature_per_month[month as usize - 1];

    let desired_temperature = match mode {
        Mode::Heating => context.indoor_temp_heating,
        Mode::Cooling => context.indoor_temp_cooling,
    };

    overall_transmission_coefficient
        * (desired_temperature - avg_month_temperature)
        * hours_in_month as f32
}

// Htr,adj
fn overall_transmission_coefficient(context: &Context) -> f32 {
    let mut sum = 0.0;

    for window in &context.building.windows {
        let area = window.height * window.width;
        let heat_transfer_coefficient = window.heat_transfer_coefficient;

        sum += area * heat_transfer_coefficient;
    }

    for wall in &context.building.walls {
        let area = wall.height * wall.width;
        let heat_transfer_coefficient = 1.0 / reduced_thermal_resistance(wall);

        sum += area * heat_transfer_coefficient;
    }

    sum
}

fn reduced_thermal_resistance(wall: &Wall) -> f32 {
    let area = wall.height * wall.width;

    area / (area * thermal_resistance_i(wall))
}

fn thermal_resistance_i(wall: &Wall) -> f32 {
    1.0 / wall.inner_surface_heat_transfer_coefficient
        + wall
            .layers
            .iter()
            .map(|layer| layer.thickness / layer.thermal_conductivity)
            .sum::<f32>()
        + 1.0 / wall.outer_surface_heat_transfer_coefficient
}

pub enum Mode {
    Heating,
    Cooling,
}
