use std::env;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {

        Ok(
            Config {
                host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
                port: env::var("PORT").unwrap_or_else(|_|  "8080".to_string()).parse().unwrap(),
            }
        )
    }
}