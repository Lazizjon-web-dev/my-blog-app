use crate::models::post::Post;
use crate::utils::auth::get_user_from_token;
use actix_web::{HttpResponse, web};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
}

pub async fn create_post(
    pool: web::Data<sqlx::PgPool>,
    form: web::Json<CreatePostRequest>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match Post::create(&pool, user.id, &form.title, &form.content).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}

pub async fn get_post(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>) -> HttpResponse {
    match Post::find_by_id(&pool, post_id.into_inner()).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Post not found"),
    }
}

pub async fn get_posts(pool: web::Data<sqlx::PgPool>) -> HttpResponse {
    match Post::find_all(&pool).await {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(_) => HttpResponse::NotFound().json("Post not found"),
    }
}
