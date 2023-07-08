use actix_web::{web, App, HttpServer};

mod models;
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().route(
            "/health-check",
            web::get().to(handlers::health::health_check),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
