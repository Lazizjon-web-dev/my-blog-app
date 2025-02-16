use crate::models::{post::Post, user::User};
use actix_web::{http::header::HeaderValue ,web, HttpResponse};
use serde::Deserialize;
use crate::utils::jwt::validate_token;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String
}

pub async fn create_post(pool: web::Data<sqlx::PgPool>, form: web::Json<CreatePostRequest>, token: HeaderValue,) -> HttpResponse {
    let email = match validate_token(&token.to_str().unwrap()) {
        Ok(claims) => claims.sub,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid token"),
    };

    let user = match User::find_by_email(&pool, &email).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json("User not found"),//? AI suggested Unauthorized I put NotFound
    };

    match Post::create(&pool, user.id, &form.title, &form.content).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}

pub async fn get_post(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>,) -> HttpResponse {
    match Post::find_by_id(&pool, post_id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Post not found"),
    }
}

pub async fn get_posts(pool: web::Data<sqlx::PgPool>, offset: web::Path<i64>) -> HttpResponse {
    match Post::find_all(&pool, 10, offset.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Post not found"),
    }
}