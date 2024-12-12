use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let port = configuration.application_port;

    run(port).await
}
