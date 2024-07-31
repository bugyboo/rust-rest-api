
use serde::Serialize;
use serde_json::Value;

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