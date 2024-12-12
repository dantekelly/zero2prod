use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscribe")]
pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
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
