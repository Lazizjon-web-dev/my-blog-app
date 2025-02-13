use super::super::handlers::auth;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(auth::login))
            .route("/register", web::post().to(auth::register)),
    );
}
