use std::env;

use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn greet_person(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be number");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(greet_person)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}