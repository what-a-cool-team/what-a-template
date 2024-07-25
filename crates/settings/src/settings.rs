use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Server {
    pub port: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
    pub connection_url: String,
    pub max_connections: u32,
    pub migrate_on_startup: bool,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    pub server: Server,
    pub database: Database,
}

impl Settings {
    pub fn from(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(path))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("__"),
            )
            .build()
            .unwrap();
        s.try_deserialize()
    }
}
