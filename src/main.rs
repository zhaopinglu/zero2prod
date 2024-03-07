// use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
// use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::*;
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration =
        get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPoolOptions::new()
        .connect_lazy_with(configuration.database.with_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address).expect("Failed to bind");
    run(listener, connection_pool)?.await
}
