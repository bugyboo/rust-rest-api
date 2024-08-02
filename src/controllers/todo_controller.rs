
use std::clone;

use actix_web::{get, post, patch, delete, HttpResponse, Responder, web};
use uuid::Uuid;

use crate::{daos::dao::*, model::{AppState, QueryOptions, Todo}};

use super::status_codes::OK;

#[get("/todos")]
pub async fn todo_list_handler( opts: web::Query<QueryOptions>, data: web::Data<AppState> ) -> impl Responder {

    let todos = data.todo_db.lock().unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListReponse {
        status: OK.to_string(),
        result: todos.len(),
        todos,
    };

    HttpResponse::Ok().json(json_response)

}

#[post("/todos")]
pub async fn create_todo_handler( mut body: web::Json<Todo>, data: web::Data<AppState> ) -> impl Responder {

    let mut vec = data.todo_db.lock().unwrap();

    let todo = vec.iter().find( |todo| todo.title == body.title );

    if todo.is_some() {
        let response = ResponseMessage {
            status: "error".to_string(),
            message: "Todo already exists".to_string(),
            payload: None,
        };
        return HttpResponse::Conflict().json(response);
    }

    let uuid_id = Uuid::new_v4();
    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(chrono::Utc::now());
    body.updatedAt = Some(chrono::Utc::now());

    let todo = body.to_owned();

    vec.push(todo);

    let response = ResponseMessage {
        status: OK.to_string(),
        message: "Todo created successfully".to_string(),
        payload: Some(serde_json::to_value(&body).unwrap()),
    };

    HttpResponse::Ok().json(response)

}

