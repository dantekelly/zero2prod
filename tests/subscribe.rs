#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use serde::Serialize;

    use zero2prod::subscribe;

    #[derive(Serialize)]
    struct CorrectFormData {
        name: String,
    }

    #[derive(Serialize)]
    struct BadFormData {
        nmea: String,
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_form(CorrectFormData {name: "Dante".to_string()})
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_missing() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_form(BadFormData { nmea: "Dante".to_string() })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_client_error());
    }
}