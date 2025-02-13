use crate::services::auth_service;
use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use serde_json::json;
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(login_data: web::Json<LoginRequest>) -> HttpResponse {
    let token = auth_service::authenticate(&login_data.email, &login_data.password);
    match token {
        Some(token) => HttpResponse::Ok().json(json!({ "token": token })),
        None => HttpResponse::Unauthorized().json("Invalid credentials"),
    }
}
