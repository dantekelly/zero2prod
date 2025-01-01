use sqlx::{Connection, Executor, PgConnection, PgPool};
use zero2prod::configuration::DatabaseSettings;

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let maintenance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        ..config.clone()
    };

    let mut connection = PgConnection::connect(&maintenance_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    connection
        .execute(format!(r#"CREATE DATABASE "{}""#, config.database_name).as_str())
        .await
        .expect("Failed to create database.");

    let connection_pool = PgPool::connect(&maintenance_settings.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}

#[cfg(test)]
mod tests {
    use super::configure_database;
    use actix_web::{body, test, web, App};
    use serde::Serialize;
    use uuid::Uuid;
    use zero2prod::configuration::get_configuration;
    use zero2prod::routes::subscribe;

    #[derive(Serialize)]
    struct FormData {
        name: String,
        email: String,
    }

    #[actix_web::test]
    async fn subscribe_returns_a_200_for_valid_form_data() {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        println!("{:?}", configuration.database.database_name);
        let connection_pool = configure_database(&configuration.database).await;
        let pool = web::Data::new(connection_pool);
        let app = test::init_service(App::new().app_data(pool.clone()).service(subscribe)).await;

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

        // Remove the database
    }

    #[actix_web::test]
    async fn subscribe_returns_a_400_when_data_is_wrong() {
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        let connection_pool = configure_database(&configuration.database).await;
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
