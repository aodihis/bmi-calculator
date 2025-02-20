use crate::handlers::api::bmi_handler::bmi as bmi_handler;
use actix_web::web;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/bmi", web::post().to(bmi_handler))
    );
}