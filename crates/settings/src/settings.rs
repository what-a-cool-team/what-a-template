use config::{Config, ConfigError};
use serde::Deserialize;
use filesystem::FileSystem;

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
    pub fn from(fs: &dyn FileSystem, path: &str) -> Result<Self, ConfigError> {
        let file_content = fs.read_file(path).map_err(|e| ConfigError::Message(e))?;

        let s = Config::builder()
            .add_source(config::File::from_str(&file_content, config::FileFormat::Toml))
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()?;

        s.try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use filesystem::MockFileSystem;

    #[test]
    fn test_settings_from() {
        // Create a mock filesystem
        let mut mock_fs = MockFileSystem::new();

        // Add a mock configuration file
        let config_content = r#"
        [server]
        port = 8080

        [database]
        connection_url = "postgres://user:password@localhost/db"
        max_connections = 5
        migrate_on_startup = true
        "#;

        mock_fs.add_file("settings.toml", config_content).unwrap();

        // Read the settings using the mock filesystem
        let settings_result = Settings::from(&mock_fs, "settings.toml");

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