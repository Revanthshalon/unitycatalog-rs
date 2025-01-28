use config::database_config::DatabaseConfig;

mod config;
mod errors;
mod persist;

fn main() {
    let app_config = DatabaseConfig::load_config();
    match app_config {
        Ok(config) => println!("{:#?}", config),
        Err(e) => println!("{:#?}", e),
    }
}
