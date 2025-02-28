use crate::handlers::posts::{CreatePostRequest, create_post, get_post, get_posts};
use actix_web::{HttpRequest, HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/posts")
            .route("", web::get().to(get_posts))
            .route("/{id}", web::get().to(get_post)),
    );
    cfg.service(web::resource("/api/posts").route(
        web::post().to(
            |req: HttpRequest,
             pool: web::Data<sqlx::PgPool>,
             form: web::Json<CreatePostRequest>| async move {
                // Extract the token from the Authorization header
                let token = match req.headers().get("Authorization") {
                    Some(header) => header.to_str().unwrap_or("").to_string(),
                    None => return HttpResponse::Unauthorized().json("Missing token"),
                };

                // Call the handler with the extracted token
                create_post(pool, form, token).await
            },
        ),
    ));
}
