use crate::models::comment::Comment;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use crate::utils::auth::get_user_from_token;

#[derive(Debug, Deserialize)]
pub struct CreateCommentRequest{
    pub content: String,
}

pub async fn create_comment(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>, form: web::Json<CreateCommentRequest>, token: String,) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
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