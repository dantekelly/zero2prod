use crate::routes::{greet_get, greet_post, healthz, subscribe};
use actix_web::{App, HttpServer};

pub async fn run(port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet_post)
            .service(greet_get)
            .service(healthz)
            .service(subscribe)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
