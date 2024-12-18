#[cfg(test)]
mod tests {
    use actix_web::{body, test, web, App};
    use zero2prod::routes::healthz;

    #[actix_web::test]
    async fn health_check_succeeds() {
        let app = test::init_service(App::new().service(healthz)).await;
        let req = test::TestRequest::get().uri("/healthz").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());

        let body = resp.into_body();
        let bytes = body::to_bytes(body).await;

        assert_eq!(bytes.unwrap(), web::Bytes::from_static(b""));
    }
}
