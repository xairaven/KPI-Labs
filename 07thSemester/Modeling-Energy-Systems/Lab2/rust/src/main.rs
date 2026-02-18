use crate::args::Args;
use crate::context::Context;
use crate::math::angle::Angle;
use crate::model::summary_radiation;
use clap::Parser;

pub mod math {
    pub mod angle;
}

pub mod args;
pub mod context;
pub mod errors;
pub mod model;

fn main() {
    let args = Args::parse();

    let mut context = Context {
        is_full_day: args.is_full_day,
        latitude: Angle::from_degree(args.latitude.replace("'", "").parse().unwrap()),
        longitude: Angle::from_degree(args.longitude.replace("'", "").parse().unwrap()),
        tilt_angle: Angle::from_degree(args.tilt_angle.replace("'", "").parse().unwrap()),
        azimuth: Angle::from_degree(args.azimuth.replace("'", "").parse().unwrap()),
        solar_irradiance: args.solar_irradiance.replace("'", "").parse().unwrap(),
        ..Default::default()
    };
    if context.is_full_day {
        let result = context.parse_day_month(&args.day, &args.month);
        if let Err(err) = result {
            panic!("{}", err.to_string())
        }
    } else {
        let result = context.parse_datetime(&args.day, &args.month, &args.time, &args.utc);
        if let Err(err) = result {
            panic!("{}", err.to_string())
        }
    }

    let summary_solar_radiation = summary_radiation(context);
    println!("{summary_solar_radiation}")
}
