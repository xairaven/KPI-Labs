use crate::errors::api::ApiError;
use crate::services::weather;
use crate::services::weather::Weather;
use std::time::{Duration, Instant};

const REFRESH_INTERVAL_SECONDS_DEFAULT: u64 = 20;

pub struct ApiState {
    pub weather: Option<Weather>,
    pub status: ApiStatus,
    pub city: String,
    pub last_updated: Instant,

    refresh_interval_seconds: u64,
}

impl Default for ApiState {
    fn default() -> Self {
        Self {
            last_updated: Instant::now()
                .checked_sub(Duration::from_secs(REFRESH_INTERVAL_SECONDS_DEFAULT))
                .unwrap(),
            city: String::new(),
            status: ApiStatus::Offline,
            weather: None,

            refresh_interval_seconds: REFRESH_INTERVAL_SECONDS_DEFAULT,
        }
    }
}

impl ApiState {
    pub fn status(&mut self) -> ApiStatus {
        if self.weather.is_some() {
            self.status = ApiStatus::Online;
        } else {
            self.status = ApiStatus::Offline;
        }

        self.status
    }

    pub fn should_update(&self) -> bool {
        self.last_updated.elapsed().as_secs() > self.refresh_interval_seconds
    }

    pub fn update(&mut self, api_key: &str, city_name: &str) -> Result<(), ApiError> {
        let results = weather::get(api_key, city_name);
        self.last_updated = Instant::now();
        self.city = city_name.to_string();
        self.status = ApiStatus::Online;

        match results {
            Ok(value) => {
                self.weather = Some(value);
                Ok(())
            },
            Err(err) => {
                self.status = ApiStatus::Offline;
                Err(err)
            },
        }
    }

    pub fn change_refresh_interval_seconds(&mut self, seconds: u64) {
        if seconds > 0 {
            self.refresh_interval_seconds = seconds;
        }
    }

    pub fn refresh_interval_seconds(&self) -> u64 {
        self.refresh_interval_seconds
    }
}

#[derive(Copy, Clone, Debug)]
pub enum ApiStatus {
    Online,
    Offline,
}
