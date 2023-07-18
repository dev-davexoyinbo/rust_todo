use actix_web::{delete, get, patch, post, web, HttpResponse, Responder, Scope};
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateTodoItemDTO {
    pub title: Option<String>,
    pub body: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateTodoItemResponseDTO {
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTodoItemResponseDTO {
    pub id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteTodoItemResponseDTO {
    pub id: u32,
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

#[patch("{id}")]
async fn update_single_todo(
    path: web::Path<usize>,
    todo_app_state: web::Data<TodoAppState>,
    data: web::Json<UpdateTodoItemDTO>
) -> impl Responder {
    let id = path.into_inner();
    let mut todo_map = todo_app_state.map.write().unwrap();
    let todo_item = todo_map.get_mut(&id);
    let data = data.into_inner();

    match todo_item {
        Some(todo_item) => {
            todo_item.body = data.body.unwrap_or(todo_item.body.clone());
            todo_item.title = data.title.or(todo_item.title.clone());
            todo_item.status = data.status.unwrap_or(todo_item.status.clone());

            HttpResponse::Ok().json(ResponseDTO::new(CreateTodoItemResponseDTO {
                id: todo_item.id
            }))
        }
        None => HttpResponse::NotFound().json(ResponseDTO::new("Todo item not found").message("Todo item not found"))
    }
}
