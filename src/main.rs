use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{name}")]
async fn greet_person(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello World!" }))
            .service(greet_person)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}