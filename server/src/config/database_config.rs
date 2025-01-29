#![allow(dead_code)]

use std::{env, sync::Arc};

use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::errors::UnityCatalogResult;

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    database_type: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    max_idle_connections: u32,
    max_open_connections: u32,
    connection_timeout: u64,
}

impl DatabaseConfig {
    pub fn load_config() -> UnityCatalogResult<Arc<Self>> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let config = Config::builder()
            .add_source(File::new("config/default", FileFormat::Toml))
            .add_source(
                File::new(&format!("config/{}", run_mode), FileFormat::Toml).required(false),
            )
            .build()?;

        let database_config = config.try_deserialize()?;
        Ok(Arc::new(database_config))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_test_config_works() {
        use std::{env, fs};

        env::set_var("RUN_MODE", "test");

        let config_content = r#" database_type = "sqlite"
username = "test_user"
password = "test_password"
host = "localhost"
port = 5432
max_idle_connections = 10
max_open_connections = 100
connection_timeout = 30"#;

        fs::create_dir_all("config").expect("Failed to create config directory");
        fs::write("config/test.toml", config_content).expect("Failed to write test config");

        let config = DatabaseConfig::load_config();

        assert!(config.is_ok());

        let config = config.unwrap();
        assert_eq!(config.database_type, "sqlite");
        assert_eq!(config.username, "test_user");
        assert_eq!(config.password, "test_password");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 5432);
        assert_eq!(config.max_idle_connections, 10);
        assert_eq!(config.max_open_connections, 100);
        assert_eq!(config.connection_timeout, 30);

        fs::remove_file("config/test.toml").expect("Failed to delete config file");
    }

    #[test]
    pub fn load_default_config_works() {
        let config = DatabaseConfig::load_config();

        assert!(config.is_ok());

        let config = config.unwrap();

        assert_eq!(config.database_type, "postgres");
        assert_eq!(config.username, "user");
        assert_eq!(config.password, "password");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert_eq!(config.max_idle_connections, 10);
        assert_eq!(config.max_open_connections, 100);
        assert_eq!(config.connection_timeout, 30);
    }
}
