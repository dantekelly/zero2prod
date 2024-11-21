use actix_web::{get, HttpResponse, Responder, HttpServer, App};


#[get("/")]
async fn hello() -> impl Responder {
    println!("Sending hello world to client");

    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}