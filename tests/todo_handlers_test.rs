use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use actix_web::{test, web, App};
use chrono::Utc;
use todo_rust::models::{
    todo_app_state::TodoAppState,
    todo_item::{TodoItem, TodoStatus}, dto_data::ResponseDTO,
};

fn get_todo_app_data() -> web::Data<TodoAppState> {
    return web::Data::new(TodoAppState {
        map: Arc::new(RwLock::new(HashMap::from([(
            1,
            TodoItem {
                id: 1,
                title: Some(String::from("Title")),
                body: "This is the body".to_string(),
                status: TodoStatus::COMPLETED,
                created_at: Utc::now().to_string(),
                updated_at: Utc::now().to_string(),
            },
        )]))),
    });
}

#[test]
async fn get_all_todos_test() {
    let app = test::init_service(
        App::new()
            .app_data(get_todo_app_data())
            .configure(todo_rust::startup::startup_app_config),
    )
    .await;
    let req = test::TestRequest::get().uri("/api/todos").to_request();
    let resp = test::call_service(&app, req).await;

    println!("Response: {:?}", resp);

    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn get_single_todo_test() {
    let app = test::init_service(
        App::new()
            .app_data(get_todo_app_data())
            .configure(todo_rust::startup::startup_app_config),
    )
    .await;
    let req = test::TestRequest::get().uri("/api/todos/1").to_request();
    let todo: ResponseDTO<TodoItem> = test::call_and_read_body_json(&app, req).await;

    assert!(todo.id > 0);
} //end function healthcheck_test

#[test]
async fn update_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::put().uri("/api/todos/1").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn delete_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::delete().uri("/api/todos/1").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn create_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::post().uri("/api/todos").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
} //end function healthcheck_test
