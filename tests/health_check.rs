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
    use uuid::Uuid;
    use zero2prod::configuration::get_configuration;
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

        // Check if the database is created and migrated
        let mut configuration = get_configuration().expect("Failed to read configuration.");
        configuration.database.database_name = Uuid::new_v4().to_string();
        let _connection_pool = configure_database(&configuration.database).await;
    }
}
