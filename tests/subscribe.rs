#[cfg(test)]
mod tests {
    use actix_web::{body, test, web, App};
    use serde::Serialize;

    use zero2prod::routes::subscribe;

    #[derive(Serialize)]
    struct FormData {
        name: String,
        email: String,
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_form(FormData {
                name: "Dante".to_string(),
                email: "theonetheonly@fakeemail.com".to_string(),
            })
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_wrong() {
        let app = test::init_service(App::new().service(subscribe)).await;
        let test_cases = vec![
            (
                FormData {
                    name: "Dante".to_string(),
                    email: "".to_string(),
                },
                "Email is missing!",
            ),
            (
                FormData {
                    name: "".to_string(),
                    email: "theonetheonly@fakeemail.com".to_string(),
                },
                "Name is missing!",
            ),
            (
                FormData {
                    name: "".to_string(),
                    email: "".to_string(),
                },
                "Missing both name and email!",
            ),
        ];

        for (invalid_form, error_message) in test_cases {
            let req = test::TestRequest::post()
                .uri("/subscribe")
                .set_form(&invalid_form)
                .to_request();
            let resp = test::call_service(&app, req).await;

            assert!(resp.status().is_client_error());

            let body = resp.into_body();
            let bytes = body::to_bytes(body).await;

            assert_eq!(
                bytes.unwrap(),
                web::Bytes::from_static(error_message.as_bytes())
            );
        }
    }
}
