use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix_web::{middleware::Logger, web, App, HttpServer};
use chrono::Utc;
use todo_rust::{
    models::{
        todo_app_state::TodoAppState,
        todo_item::{TodoItem, TodoStatus},
    },
    startup,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo = TodoItem {
        id: 1,
        title: Some(String::from("Title")),
        body: "This is the body".to_string(),
        status: TodoStatus::COMPLETED,
        created_at: Utc::now().to_string(),
        updated_at: Utc::now().to_string(),
    };

    let todo_app_data = web::Data::new(TodoAppState {
        map: Arc::new(RwLock::new(HashMap::from([(todo.id as usize, todo)]))),
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(todo_app_data.clone())
            .configure(startup::startup_app_config)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
