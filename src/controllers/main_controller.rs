
use actix_web::{get, HttpResponse, Responder};

use crate::daos::dao::{ResponseMessage, UserInfoDao};

use super::status_codes::OK;

#[get("/")]
pub async fn root() -> impl Responder {

    let user = UserInfoDao {
        id: 1,
        name: "John Doe".to_string(),
        email: "".to_string(),
    };
        let resp_message = &ResponseMessage {
            status: OK.to_string(),
            message: "Welcome to NerveLife Service".to_string(),
            payload: Some(serde_json::to_value(&user).unwrap()),
        };
    HttpResponse::Ok().json(resp_message)
}

#[get("/health")]
pub async fn health_handler() -> impl Responder {
    const MESSAGE: &str = "NerveLife Service is running";

    let resp_message = &ResponseMessage {
        status: OK.to_string(),
        message: MESSAGE.to_string(),
        payload: None,
    };
    HttpResponse::Ok().json(resp_message)
}