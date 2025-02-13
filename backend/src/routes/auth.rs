use actix_web::web;
use super::handlers::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/auth")
            .route("/login", web::post().to(auth::login))
            .route("/register", web::post().to(auth::register)),
    );
}
