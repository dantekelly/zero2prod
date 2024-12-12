use actix_web::{post, web, HttpResponse};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
struct FormData {
    email: String,
    name: String,
}

#[post("/subscribe")]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    println!("Attempting to connect to the database...");

    dbg!(&form);
    if form.name.is_empty() && form.email.is_empty() {
        return HttpResponse::BadRequest().body("Missing both name and email!");
    };
    if form.name.is_empty() {
        return HttpResponse::BadRequest().body("Name is missing!");
    };
    if form.email.is_empty() {
        return HttpResponse::BadRequest().body("Email is missing!");
    };
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await
    {
        Ok(_) => HttpResponse::Ok().body(format!("Welcome {}!", form.name)),
        Err(e) => {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().body("Failed to save new subscriber.")
        }
    }
    // HttpResponse::Ok().body(format!("Welcome {}!", form.name))
}
