use actix_web::{App, HttpServer};
use todo_rust::startup;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(startup::startup_app_config))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
