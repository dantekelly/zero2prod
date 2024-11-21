use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, HttpServer, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    println!("[INFO] Sending hello world to client");

    HttpResponse::Ok().body("Hello, world!")
}

#[post("/greet")]
async fn greet(_: HttpRequest, json: web::Json<Greeting>) -> impl Responder {
    let name = json.name.clone();
    println!("[INFO] Greeting {} with hello", name); // TODO: Sanitize input
    format!("Hello, {}!", name)
}

#[get("/greet/{name}")]
async fn greet_route(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();

    if name.is_empty() {
        println!("[ERROR] Something went wrong!");
        return "Unexpected error has occured.".to_string()
    }

    println!("[INFO] Greeting {} with hello", name);
    format!("[DEPRECATED] Hello, {}!", name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .service(greet_route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}