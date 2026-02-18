use thiserror::Error;

#[derive(Error, Debug)]
pub enum EnvError {
    #[error("Config is not loaded.")]
    ConfigNotLoaded,

    #[error("OpenWeatherMap API Key not loaded.")]
    APIKeyNotLoaded,
}
