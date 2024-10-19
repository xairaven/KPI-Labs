use crate::context::Context;
use crate::math::angle::Angle;
use chrono::{Datelike, Timelike};

pub const SOLAR_CONST: f64 = 1367.0;

pub fn summary_radiation(context: Context) -> f64 {
    let a = f64::sin(context.latitude.radian()) * f64::cos(context.tilt_angle.radian());
    let b = f64::cos(context.latitude.radian())
        * f64::sin(context.tilt_angle.radian())
        * f64::cos(context.azimuth.radian());
    let c = f64::sin(context.tilt_angle.radian()) * f64::sin(context.azimuth.radian());
    let d = f64::cos(context.latitude.radian()) * f64::cos(context.tilt_angle.radian());
    let e = f64::sin(context.latitude.radian())
        * f64::sin(context.tilt_angle.radian())
        * f64::cos(context.azimuth.radian());

    let b0_coef: f64 = 360.0 / 365.0;

    let b_coef = b0_coef * (context.datetime.ordinal() as f64 - 81.0);

    let time_correction = 1.0 / 60.0
        * (9.87 * f64::sin(2.0 * b_coef) - 7.53 * f64::cos(b_coef) - 1.5 * f64::sin(b_coef));

    let hour_angle = Angle::from_degree(
        15.0 * (context.datetime.hour() as f64
            - 12.0
            - time_correction
            - context.datetime.offset().local_minus_utc() as f64 / 60.0 / 60.0
            + context.longitude.degree()),
    );

    let solar_declination =
        Angle::from_degree(23.45 * f64::sin(b0_coef * (context.datetime.ordinal() as f64 + 284.0)));

    let incident_angle = (a - b) * f64::sin(solar_declination.radian())
        + (c * f64::sin(hour_angle.radian()) + (d * e) * f64::cos(hour_angle.radian()))
            * f64::cos(solar_declination.radian());
    let incident_angle = Angle::from_radian(f64::acos(incident_angle));

    let zenithal = Angle::from_radian(f64::acos(
        f64::sin(context.latitude.radian()) * f64::sin(solar_declination.radian())
            + f64::cos(context.latitude.radian())
                * f64::cos(solar_declination.radian())
                * f64::cos(hour_angle.radian()),
    ));

    let post_atmospheric_rad = SOLAR_CONST
        * (1.0 + 0.033 * f64::cos(b0_coef * context.datetime.ordinal() as f64))
        * f64::cos(zenithal.radian());

    let i_total_horizontal = context.solar_irradiance;
    let kt_index = i_total_horizontal / post_atmospheric_rad;

    let i_diffuse_horizontal = i_total_horizontal / (1.0 + f64::exp(-5.0 + 8.6 * kt_index));
    let i_reflected_horizontal = context.albedo * i_total_horizontal;
    let i_direct_horizontal = i_total_horizontal - i_diffuse_horizontal - i_reflected_horizontal;

    i_direct_horizontal * (f64::cos(incident_angle.radian()) / f64::cos(zenithal.radian()))
        + i_diffuse_horizontal * (1.0 + f64::cos(context.tilt_angle.radian())) / 2.0
        + i_reflected_horizontal * (1.0 - f64::cos(context.tilt_angle.radian())) / 2.0
}
