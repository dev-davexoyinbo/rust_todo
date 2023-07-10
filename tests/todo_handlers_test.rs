use actix_web::{test, App};

#[test]
async fn get_all_todos_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::get().uri("/todos").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn get_single_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::get().uri("/todos/1").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
} //end function healthcheck_test



#[test]
async fn update_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::put().uri("/todos/1").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn delete_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::delete().uri("/todos/1").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
} //end function healthcheck_test

#[test]
async fn create_todo_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::post().uri("/todos").to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
} //end function healthcheck_test
