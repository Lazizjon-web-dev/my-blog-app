use crate::models::{like::Like, user::User};
use actix_web::{http::header::HeaderValue, web, HttpResponse};
use crate::utils::jwt::validate_token;

pub async fn like_post(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>, token: HeaderValue) -> HttpResponse {
    let email = match validate_token(&token.to_str().unwrap()) {
        Ok(claims) => claims.sub,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid token"),
    };

    let user = match User::find_by_email(&pool, &email).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json("User not found"),
    };

    match Like::create(&pool, user.id, post_id.into_inner()).await {
        Ok(like) => HttpResponse::Ok().json(like),
        Err(_) => return HttpResponse::InternalServerError().json("Failed to like post"),
    }
}
//TODO: change name to dislike_post
pub async fn unlike_post(pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>, token: HeaderValue) -> HttpResponse {
    let email = match validate_token(&token.to_str().unwrap()) {
        Ok(claims) => claims.sub,
        Err(_) => return HttpResponse::Unauthorized().json("Invalid token") 
    };

    let user = match User::find_by_email(&pool, &email).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::NotFound().json("User not found"),
    };

    match Like::find(&pool, user.id, post_id.into_inner()).await {
        Ok(Some(like)) => {
            match like.delete(&pool).await {
                Ok(_) => HttpResponse::Ok().json("Like removed."),
                Err(_) => HttpResponse::InternalServerError().json("Failed to remove like"),
            }
        },
        Ok(None) => HttpResponse::NotFound().json("Like not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to find like"),
    }
}