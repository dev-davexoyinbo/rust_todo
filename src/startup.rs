use actix_web::web;

use crate::handlers;

pub fn startup_app_config(cfg: &mut web::ServiceConfig) {
    cfg.route(
        "/health-check",
        web::get().to(handlers::health::health_check),
    );
}
