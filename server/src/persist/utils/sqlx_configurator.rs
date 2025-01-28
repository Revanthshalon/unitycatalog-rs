#![allow(dead_code)]
use crate::config::DatabaseConfig;
use sqlx::{Pool, Postgres};

pub struct SqlxConfigurator {
    pub db_pool: Pool<Postgres>,
}

impl SqlxConfigurator {
    pub fn new(_database_config: DatabaseConfig) -> Self {
        todo!()
    }
}
