//! # Main
//!
//! This is a main crate of app.

mod routes;
mod config;
mod utils;
mod handlers;
mod models;

use crate::config::Config;
use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;

async fn bmi() -> impl Responder {
    "Hello worldx!"
}

// Define a struct for the JSON response
#[derive(Serialize)]
struct ErrorResponse {
    status: String,
    message: String,
}

async fn not_found() -> actix_web::Result<HttpResponse> {
    let error_response = ErrorResponse {
        status: "error".to_string(),
        message: "Not found".to_string(),
    };

    let response = HttpResponse::build(StatusCode::NOT_FOUND)
        .json(error_response);

    Ok(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::load().expect("Failed to load configuration");

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .route("/bmi", web::get().to(bmi)),
            )
            .default_service(web::route().to(not_found))
    })
    .bind((config.host, config.port))?
    .run()
    .await
}
