use crate::models::user::User;
use crate::utils::jwt::create_token;
use actix_web::{HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}
#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    token: String,
}

pub async fn register(
    pool: web::Data<sqlx::PgPool>,
    form: web::Json<RegisterRequest>,
) -> HttpResponse {
    let password_hash = hash(&form.password, DEFAULT_COST).unwrap();
    match User::create(&pool, &form.username, &form.email, &password_hash).await {
        Ok(_) => HttpResponse::Ok().json("User registered succesfully!"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to register user"),
    }
}
pub async fn login(pool: web::Data<sqlx::PgPool>, form: web::Json<LoginRequest>) -> HttpResponse {
    match User::find_by_email(&pool, &form.email).await {
        Ok(user) => {
            if verify(&form.password, &user.password_hash).unwrap() {
                let token = create_token(&user.email);
                HttpResponse::Ok().json(AuthResponse { token })
            } else {
                HttpResponse::Unauthorized().json("Invalid credetials")
            }
        }
        //TODO: Change the error reponse messages, now it returns same message no matter which credential is invalid
        Err(_) => HttpResponse::Unauthorized().json("Invalid credetials"),
    }
}
