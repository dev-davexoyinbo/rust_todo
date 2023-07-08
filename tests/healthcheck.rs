use actix_web::{test, App};

#[actix_web::test]
async fn healthcheck_test() {
    let app =
        test::init_service(App::new().configure(todo_rust::startup::startup_app_config)).await;
    let req = test::TestRequest::get().uri("/health-check").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
} //end function healthcheck_test
