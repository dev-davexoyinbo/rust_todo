use actix_web::{web};

use crate::handlers::{self, todo_handlers};

pub fn startup_app_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(todo_handlers::handler_service_scope())
        .route(
            "/health-check",
            web::get().to(handlers::health::health_check),
        );
    
}
