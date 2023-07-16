use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};

use crate::models::{
    paginated_data::{PaginatedData, PaginationQuery},
    todo_app_state::TodoAppState,
    todo_item::TodoItem,
};

pub fn handler_service_scope() -> Scope {
    return web::scope("/api/todos").service(get_all);
}

#[get("")]
async fn get_all(
    todo_app_state: web::Data<TodoAppState>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let todo_map = todo_app_state.map.lock().unwrap();
    let results: Vec<TodoItem> = todo_map.values().cloned().collect();

    let paginated_response =
        PaginatedData::<TodoItem>::from_data(results, query.into_inner(), todo_map.len());

    return HttpResponse::Ok().json(paginated_response);
} // end method get_all

#[post("")]
async fn create_todo_item() -> impl Responder {
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
