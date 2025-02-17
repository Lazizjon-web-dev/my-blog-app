use crate::models::like::Like;
use crate::utils::auth::get_user_from_token;
use actix_web::{HttpResponse, web};

pub async fn like_post(
    pool: web::Data<sqlx::PgPool>,
    post_id: web::Path<i32>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match Like::create(&pool, user.id, post_id.into_inner()).await {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to like post"),
    }
}
//TODO: change name to dislike_post
pub async fn unlike_post(
    pool: web::Data<sqlx::PgPool>,
    post_id: web::Path<i32>,
    token: String,
) -> HttpResponse {
    let user = match get_user_from_token(&pool, &token).await {
        Ok(user) => user,
        Err(response) => return response,
    };

    match Like::find(&pool, user.id, post_id.into_inner()).await {
        Ok(Some(like)) => match like.delete(&pool).await {
            Ok(_) => HttpResponse::Ok().json("Like removed."),
            Err(_) => HttpResponse::InternalServerError().json("Failed to remove like"),
        },
        Ok(None) => HttpResponse::NotFound().json("Like not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to find like"),
    }
}
