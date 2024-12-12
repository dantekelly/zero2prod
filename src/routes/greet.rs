use actix_web::{get, post, web, HttpRequest, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct Greeting {
    name: String,
}

#[post("/greet")]
pub async fn greet_post(_: HttpRequest, json: web::Json<Greeting>) -> impl Responder {
    let name = json.name.clone();
    println!("[INFO] Greeting {} with hello", name); // TODO: Sanitize input
    format!("Hello, {}!", name)
}

#[get("/greet/{name}")]
pub async fn greet_get(path: web::Path<String>) -> impl Responder {
    let name = path.into_inner();

    if name.is_empty() {
        println!("[ERROR] Something went wrong!");
        return "Unexpected error has occured.".to_string();
    }

    println!("[INFO] Greeting {} with hello", name);
    format!("[DEPRECATED] Hello, {}!", name)
}
