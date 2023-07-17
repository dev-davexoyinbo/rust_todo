use actix_web::{Responder, HttpResponse};
use chrono::Utc;
use serde::Serialize;

use crate::models::dto_data::ResponseDTO;

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

    return HttpResponse::Ok().json(ResponseDTO::new(data));
}
