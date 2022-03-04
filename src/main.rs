use std::env;

use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;

use env_logger::Env;

#[get("/{name}")]
async fn greet_person(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let port = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT must be number");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(greet_person)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
