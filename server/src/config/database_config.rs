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
            .add_source(File::new("server/config/default", FileFormat::Toml))
            .add_source(
                File::new(&format!("server/config/{}", run_mode), FileFormat::Toml).required(false),
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
    pub fn load_config_works() {
        todo!()
    }

    #[test]
    pub fn load_config_missing_file() {
        let config = DatabaseConfig::load_config();
        assert!(config.is_err())
    }
}
