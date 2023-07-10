use actix_web::{get, web, HttpResponse, Responder, Scope, delete, put};

pub fn handler_service_scope() -> Scope {
    return web::scope("/todos").service(get_all);
}

#[get("")]
async fn get_all() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}

#[get("{id}")]
async fn get_single_todo() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}

#[delete("{id}")]
async fn delete_single_todo() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}

#[put("{id}")]
async fn update_single_todo() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}
