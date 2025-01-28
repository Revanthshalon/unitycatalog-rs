#![allow(dead_code)]
use sqlx::{Pool, Postgres};

pub struct SqlxConfigurator {
    pub db_pool: Pool<Postgres>,
}
