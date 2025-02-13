use actix_cors::Cors;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/api/posts")]
async fn get_posts() -> impl Responder {
    HttpResponse::Ok().json(vec![
        Post { id: 1, title: "First Post".to_string() },
        Post { id: 2, title: "Second Post".to_string() },
    ])
}

#[derive(serde::Serialize)]
struct Post {
    id: u32,
    title: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173") // Allow requests from your frontend
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec!["Content-Type", "Authorization"])
                    .max_age(3600),
            )
            .service(get_posts)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
