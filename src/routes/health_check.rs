use actix_web::{get, HttpResponse, Responder};

// TODO: Create a Health Check SAAS for internal usage.
#[get("/healthz")]
pub async fn healthz() -> impl Responder {
    println!("[INFO] We are sending a health check to client!");
    HttpResponse::Ok()
}
