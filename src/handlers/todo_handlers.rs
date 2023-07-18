use actix_web::{delete, get, post, put, web, HttpResponse, Responder, Scope};
use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::models::{
    dto_data::ResponseDTO,
    paginated_data::{PaginatedData, PaginationQuery},
    todo_app_state::TodoAppState,
    todo_item::{TodoItem, TodoStatus},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoItemDTO {
    pub title: Option<String>,
    pub body: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoItemResponseDTO {
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteTodoItemResponseDTO {
    pub id: u32,
}

impl CreateTodoItemDTO {
    fn create_todo_item_with_id(self, id: u32) -> TodoItem {
        return TodoItem {
            id,
            title: self.title,
            body: self.body,
            status: TodoStatus::PENDING,
            created_at: Utc::now().to_string(),
            updated_at: Utc::now().to_string(),
        };
    }
}

pub fn handler_service_scope() -> Scope {
    return web::scope("/api/todos")
        .service(get_all)
        .service(get_single_todo)
        .service(create_todo_item)
        .service(delete_single_todo)
        .service(update_single_todo);
}

#[get("")]
async fn get_all(
    todo_app_state: web::Data<TodoAppState>,
    query: web::Query<PaginationQuery>,
) -> impl Responder {
    let todo_map = todo_app_state.map.read().unwrap();
    let results: Vec<TodoItem> = todo_map.values().cloned().collect();

    let paginated_response =
        PaginatedData::<TodoItem>::from_data(results, query.into_inner(), todo_map.len());

    return HttpResponse::Ok().json(paginated_response);
} // end method get_all

#[post("")]
async fn create_todo_item(
    data: web::Json<CreateTodoItemDTO>,
    todo_app_state: web::Data<TodoAppState>,
) -> impl Responder {
    let max_id = todo_app_state.get_max_id();
    let new_id = match max_id {
        Some(id) => id + 1,
        None => 1,
    };
    let todo_item = data.into_inner().create_todo_item_with_id(new_id as u32);
    todo_app_state.push(todo_item);

    return HttpResponse::Created().json(ResponseDTO::new(CreateTodoItemResponseDTO {
        id: new_id as u32,
    }));
}

#[get("/{id}")]
async fn get_single_todo(
    path: web::Path<usize>,
    todo_app_data: web::Data<TodoAppState>,
) -> impl Responder {
    let id = path.into_inner();
    let todo_map = todo_app_data.map.read().unwrap();
    return match todo_map.get(&id) {
        Some(todo) => HttpResponse::Ok().json(ResponseDTO::new(todo)),
        None => HttpResponse::NotFound()
            .json(ResponseDTO::new("Todo item not found.").message("Todo item not found.")),
    };
}

#[delete("{id}")]
async fn delete_single_todo(
    path: web::Path<usize>,
    todo_app_state: web::Data<TodoAppState>,
) -> impl Responder {
    let id = path.into_inner();
    match todo_app_state.delete_todo_with_id(id) {
        Some(todo_item) => HttpResponse::Ok().json(ResponseDTO::new(DeleteTodoItemResponseDTO {
            id: todo_item.id as u32,
        })),
        None => HttpResponse::NotFound()
            .json(ResponseDTO::new("Todo item not found").message("Todo item not found")),
    }
}

#[put("{id}")]
async fn update_single_todo() -> impl Responder {
    HttpResponse::InternalServerError().finish()
}
