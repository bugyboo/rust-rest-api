
use serde::Serialize;
use serde_json::Value;
use crate::model::Todo;

#[derive(Serialize)]
pub struct ResponseMessage {
    pub status: String,
    pub message: String,
    pub payload: Option<Value>,
}

#[derive(Serialize)]
pub struct UserInfoDao {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct TodoDao {
    pub todo: Todo,
}

#[derive(Serialize)]
pub struct SingTodoDao {
    pub status: String,
    pub todo: Todo,
}

#[derive(Serialize)]
pub struct TodoListReponse {
    pub status: String,
    pub result: usize,
    pub todos: Vec<Todo>,
}

