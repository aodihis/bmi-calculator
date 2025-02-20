//! # Main
//!
//! This is a main crate of app.

mod routes;
mod config;
mod utils;
mod handlers;
mod models;
mod errors;

use crate::config::Config;
use actix_web::{middleware, web, App, HttpServer};
use log::info;
use crate::handlers::error_handler::{handle_json_error, not_found};
use crate::routes::bmi_route::init as bmi_routes_init;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::load().expect("Failed to load configuration");

    info!("Listening for incoming connections on {}:{}", config.host, config.port);
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default()
                .error_handler(handle_json_error))
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .configure(bmi_routes_init)
            .default_service(web::route().to(not_found))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
