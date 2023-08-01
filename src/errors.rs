use actix_web::{HttpResponse};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HttpError {
    pub message: String,
    pub code: i32,
}

pub fn login_incorrect() -> HttpResponse {
    let response = HttpError {
        message: "Username or password is wrong".to_string(),
        code: 404,
    };
    return HttpResponse::NotFound().json(response);
}

pub fn internal_server_error(err: &str) -> HttpResponse {
    let response = HttpError {
        message: err.to_string(),
        code: 500,
    };
    return HttpResponse::NotFound().json(response);
}