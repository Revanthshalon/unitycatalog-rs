mod config;
mod errors;
mod persist;

use config::DatabaseConfig;

fn main() {
    let app_config =
        DatabaseConfig::load_config(Some("server/config/default".to_string())).unwrap();
    println!("{}", app_config.get_connection_string().unwrap());
}
