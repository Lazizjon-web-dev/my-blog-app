use crate::handlers::likes::{like_post, unlike_post};
use actix_web::{HttpRequest, HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("api/posts/{post_id}/likes")
            .route(web::post().to(|req: HttpRequest, pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>| async move {
                // Extract the token from the Authorization header
                let token = match req.headers().get("Authorization") {
                    Some(header) => header.to_str().unwrap_or("").to_string(),
                    None => return HttpResponse::Unauthorized().json("Missing token"),
                };

                // Call the handler with the extracted token
                like_post(pool, post_id, token).await
            }))
            .route(web::delete().to(|req: HttpRequest, pool: web::Data<sqlx::PgPool>, post_id: web::Path<i32>| async move {
                // Extract the token from the Authorization header
                let token = match req.headers().get("Authorization") {
                    Some(header) => header.to_str().unwrap_or("").to_string(),
                    None => return HttpResponse::Unauthorized().json("Missing token"),
                };

                // Call the handler with the extracted token
                unlike_post(pool, post_id, token).await
            })),
    );
}
