use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder, HttpServer, App};
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

// TODO: Create a Health Check SAAS for internal usage.
#[get("/healthz")]
pub async fn health_check() -> impl Responder {
    println!("[INFO] We are sending a health check to client!");
    HttpResponse::Ok()
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

#[post("/subscribe")]
async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    if form.name.is_empty() && form.email.is_empty() {
        return HttpResponse::BadRequest().body("Missing both name and email!");
    };
    if form.name.is_empty() {
        return HttpResponse::BadRequest().body("Name is missing!");
    };
    if form.email.is_empty() {
        return HttpResponse::BadRequest().body("Email is missing!");
    };

    HttpResponse::Ok().body(format!("Welcome {}!", form.name))
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(greet)
            .service(greet_route)
            .service(health_check)
            .service(subscribe)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}