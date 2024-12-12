use crate::routes::{greet_get, greet_post, healthz, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .service(greet_post)
            .service(greet_get)
            .service(healthz)
            .service(subscribe)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
