use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(long)]
    pub is_full_day: bool,

    #[arg(long)]
    pub day: String,

    #[arg(long)]
    pub month: String,

    #[arg(long, default_value = "12:00")]
    pub time: String,

    #[arg(long, default_value = "+02:00")]
    pub utc: String,

    #[arg(long, default_value = "50.45")]
    pub latitude: String,

    #[arg(long, default_value = "30.52")]
    pub longitude: String,

    #[arg(long, default_value = "30.0")]
    pub tilt_angle: String,

    #[arg(long, default_value = "42.0")]
    pub azimuth: String,

    #[arg(long, default_value = "3.1")]
    pub solar_irradiance: String,

    #[arg(long, default_value = "0.2")]
    pub albedo: String,
}
