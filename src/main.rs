//! # Main
//!
//! This is a main crate of app.

use actix_web::{middleware, web, App, HttpServer, Responder};

async fn bmi() -> impl Responder {
    "Hello worldx!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(
                // prefixes all resources and routes attached to it...
                web::scope("/api")
                    // ...so this handles requests for `GET /app/index.html`
                    .route("/bmi", web::get().to(bmi)),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
