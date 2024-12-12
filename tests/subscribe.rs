#[cfg(test)]
mod tests {
    use actix_web::{body, test, web, App};
    use serde::Serialize;
    use sqlx::PgPool;
    use zero2prod::configuration::get_configuration;
    use zero2prod::routes::subscribe;

    #[derive(Serialize)]
    struct FormData {
        name: String,
        email: String,
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_pool = PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");
        let pool = web::Data::new(connection_pool);
        let app = test::init_service(App::new().app_data(pool.clone()).service(subscribe)).await;

        sqlx::query!("TRUNCATE TABLE subscriptions")
            .execute(pool.get_ref())
            .await
            .expect("Failed to clean database");

        // Request
        let req = test::TestRequest::post()
            .uri("/subscribe")
            .set_form(FormData {
                name: "Dante".to_string(),
                email: "theonetheonly@fakeemail.com".to_string(),
            })
            .to_request();

        // Response
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(pool.get_ref())
            .await
            .expect("Failed to fetch saved subscription.");

        assert_eq!(saved.name, "Dante");
        assert_eq!(saved.email, "theonetheonly@fakeemail.com");
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_wrong() {
        let configuration = get_configuration().expect("Failed to read configuration.");
        let connection_pool = PgPool::connect(&configuration.database.connection_string())
            .await
            .expect("Failed to connect to Postgres.");
        let pool = web::Data::new(connection_pool);
        let app = test::init_service(App::new().app_data(pool.clone()).service(subscribe)).await;
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

            dbg!(&resp.status());
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
