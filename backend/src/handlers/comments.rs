use crate::models::{comment::Comment, user::User};
use actix_web::{http::header::HeaderValue, web, HttpResponse};
use serde::Deserialize;
use crate::utils::jwt::validate_token;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest{
    pub content: String,
}

pub async fn create_comment(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>, form: web::Json<CreateCommentRequest>, token: HeaderValue,) -> HttpResponse {
    let email = match validate_token(&token.to_str().unwrap()) {
        Ok(claims) => claims.sub,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid token"),
    };

    let user = match User::find_by_email(&pool, &email).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json("User not found") //? AI suggested Unauthorized I put NotFound
    };

    match Comment::create(&pool, user.id, post_id.into_inner(), &form.content).await {
        Ok(comment) => HttpResponse::Ok().json(comment),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create comment"),
    }
}

pub async fn get_comments(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>,) -> HttpResponse {
    match Comment::find_by_post_id(&pool, post_id.into_inner()).await {
        Ok(comments) => HttpResponse::Ok().json(comments),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch comments"),
    }
}