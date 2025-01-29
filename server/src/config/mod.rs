#![allow(dead_code)]
pub mod database_config;

pub use self::database_config::DatabaseConfig;

pub struct AppConfig {
    pub database_config: DatabaseConfig,
}

impl AppConfig {
    pub fn new(_path_option: Option<String>) -> Self {
        todo!()
    }
}
