use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, HttpServer, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("Sending hello world to client");

    HttpResponse::Ok().body("Hello, world!")
}

#[post("/greet")]
async fn greet(_: HttpRequest, json: web::Json<Greeting>) -> impl Responder {
    let name = json.name.clone();
    format!("Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}