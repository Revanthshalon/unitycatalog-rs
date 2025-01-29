#![allow(dead_code)]

use std::{env, sync::Arc, time::Duration};

use config::{Config, File, FileFormat};
use serde::Deserialize;

use crate::errors::{UnityCatalogError, UnityCatalogResult};

#[derive(Debug, Deserialize)]
pub struct DatabaseConfig {
    database_type: String,
    username: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
    max_idle_connections: u32,
    max_open_connections: u32,
    connection_timeout: u64,
}

impl DatabaseConfig {
    pub fn load_config(path_option: Option<String>) -> UnityCatalogResult<Arc<Self>> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        let path = path_option.unwrap_or_else(|| "config/default".into());

        let config = Config::builder()
            .add_source(File::new(&path, FileFormat::Toml))
            .add_source(
                File::new(&format!("config/{}", run_mode), FileFormat::Toml).required(false),
            )
            .build()?;

        let database_config = config.try_deserialize()?;
        Ok(Arc::new(database_config))
    }

    pub fn get_connection_string(&self) -> UnityCatalogResult<String> {
        match self.database_type.as_str() {
            "postgres" => Ok(format!(
                "postgres://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            )),
            "mysql" => Ok(format!(
                "mysql://{}:{}@{}:{}/{}",
                self.username, self.password, self.host, self.port, self.database_name
            )),
            _ => Err(UnityCatalogError::Unsupported(format!(
                "Unsupported Database: {}",
                self.database_type
            ))),
        }
    }

    pub fn get_max_idle_connections(&self) -> u32 {
        self.max_idle_connections
    }

    pub fn get_max_open_connections(&self) -> u32 {
        self.max_open_connections
    }

    pub fn get_timeout(&self) -> Duration {
        Duration::from_secs(self.connection_timeout)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn load_default_config_works() {
        let config = DatabaseConfig::load_config(None);

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
