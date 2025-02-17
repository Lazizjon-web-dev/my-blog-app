use crate::handlers::comments::{CreateCommentRequest, create_comment, get_comments};
use actix_web::{HttpRequest, HttpResponse, web};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/posts/{post_id}/comments").route("", web::get().to(get_comments)));
    cfg.service(
        web::resource("api/posts/{post_id}/comments").route(web::post().to(
            |req: HttpRequest,
             pool: web::Data<sqlx::PgPool>,
             post_id: web::Path<i32>,
             form: web::Json<CreateCommentRequest>| async move {
                // Extract the token from the Authorization header
                let token = match req.headers().get("Authorization") {
                    Some(header) => header.to_str().unwrap_or("").to_string(),
                    None => return HttpResponse::Unauthorized().json("Missing token"),
                };

                // Call the handler with the extracted token
                create_comment(pool, post_id, form, token).await
            },
        )),
    );
}
