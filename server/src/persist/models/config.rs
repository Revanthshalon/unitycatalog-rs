#![allow(dead_code)]

#[derive(Debug)]
pub struct DatabaseConfig {
    database_type: String,
    username: String,
    passsword: String,
    host: String,
    port: u16,
    max_idle_connections: u32,
    max_open_connections: u32,
    connection_timeout: u64,
}
