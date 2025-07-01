mod config;
use config::Config;

fn main() {
    match Config::from_env() {
        Ok(_cfg) => {
            println!("Configuration loaded successfully.");
            // Use _cfg.api_key, _cfg.database_url as needed
        }
        Err(e) => {
            eprintln!("Configuration error: {}", e);
            std::process::exit(1);
        }
    }
}
