use actix_web::{web, Responder};
use chrono::Utc;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct HealthCheckDTO {
    message: String,
    time: String,
}
pub async fn health_check() -> impl Responder {
    let data = HealthCheckDTO {
        message: String::from("This server is alive"),
        time: Utc::now().to_string(),
    };

    web::Json(data)
}