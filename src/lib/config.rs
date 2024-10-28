use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Database {
    url: String,
}

#[derive(Deserialize)]
pub struct Config {
    database: Database,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::init().expect("Config init failure"));

impl Config {
    pub fn init() -> Result<Self, ConfigError> {
        let mut builder = config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::default().separator("_"));
        builder.build()?.try_deserialize()
    }
}
