use super::dto;
use super::services;
use crate::errors;
use crate::DbPool;
use actix_web::HttpRequest;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/register")]
async fn register(
    pool: web::Data<DbPool>,
    payload: web::Json<dto::RegisterPayload>,
) -> impl Responder {
    let _payload = payload.into_inner();
    let result = services::add_user(
        &_payload.username,
        &_payload.email,
        &_payload.password,
        &pool.get().unwrap(),
    );

    match result {
        Ok(user) => {
            let serialized = serde_json::to_value(&user).unwrap();
            let response: dto::UserResponse = serde_json::from_value(serialized).unwrap();
            HttpResponse::Ok().json(response)
        }
        Err(err) => errors::internal_server_error(&err.to_string()),
    }
}

#[post("/login")]
async fn login(
    req: HttpRequest,
    pool: web::Data<DbPool>,
    payload: web::Json<dto::LoginPayload>,
) -> impl Responder {
    let _payload = payload.into_inner();
    let result = services::get_user_by_username(&_payload.username, &pool.get().unwrap());
    match result {
        Ok(user) => {
            if !user
                .password_hash
                .eq(&format!("{:x}", md5::compute(_payload.password)))
            {
                return errors::login_incorrect();
            }

            let serialized_user = serde_json::to_value(&user).unwrap();
            let user_response: dto::UserResponse = serde_json::from_value(serialized_user).unwrap();
            
            // Response (Token and user infos)
            let response = dto::LoginResponse {
                token: "".to_string(),
                user: user_response
            };

            // Remember me
            if _payload.remember_me {
                let ip_address = if let Some(val) = req.peer_addr() {
                    val.ip().to_string()
                } else {
                    "".to_string()
                };

                let result = services::add_user_login(
                    &user.id,
                    &_payload.device_id,
                    &ip_address,
                    &pool.get().unwrap(),
                );

                if result.is_err() {
                    return errors::internal_server_error(&result.err().unwrap().to_string());
                }
            }
            HttpResponse::Ok().json(response)
        }
        Err(_) => errors::login_incorrect(),
    }
}
