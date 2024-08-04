use config::{Config, ConfigError};
use serde::Deserialize;
use filesystem::{FileSource};

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
    pub fn from(file_source: &FileSource) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(config::File::from_str(&file_source.content, config::FileFormat::Toml))
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

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::{InMemoryFileSystem};

    #[test]
    fn test_settings_from() {
        // Create an in-memory filesystem
        let fs = InMemoryFileSystem::new();

        // Add a mock configuration file
        let config_content = r#"
        [server]
        port = 8080

        [database]
        connection_url = "postgres://user:password@localhost/db"
        max_connections = 5
        migrate_on_startup = true
        "#;

        let file_source = FileSource::new(String::from(config_content));
        // Read the settings using the mock filesystem
        let settings_result = Settings::from(&file_source);

        match settings_result {
            Ok(settings) => {
                assert_eq!(settings.server.port, 8080);
                assert_eq!(settings.database.connection_url, "postgres://user:password@localhost/db");
                assert_eq!(settings.database.max_connections, 5);
                assert!(settings.database.migrate_on_startup);
            }
            Err(e) => panic!("Failed to deserialize settings: {}", e),
        }
    }
}