mod config;
mod errors;
mod persist;

use config::DatabaseConfig;

fn main() {
    let app_config = DatabaseConfig::load_config();
    match app_config {
        Ok(config) => println!("{:#?}", config),
        Err(e) => println!("{:#?}", e),
    }
}
