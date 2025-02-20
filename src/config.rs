use dotenv::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
}
impl Config {
    pub fn load() -> Result<Self, Error> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("invalid port value"),
        })
    }
}
