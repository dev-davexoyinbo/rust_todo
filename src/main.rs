use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix_web::{web, App, HttpServer};
use todo_rust::{startup, models::todo_app_state::TodoAppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_app_data = web::Data::new(TodoAppState {
        map: Arc::new(Mutex::new(HashMap::new())),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(todo_app_data.clone())
            .configure(startup::startup_app_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
