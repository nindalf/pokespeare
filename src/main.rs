use std::net::TcpListener;

use sqlx::PgPool;

mod configuration;
mod routes;
mod startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("localhost:{}", settings.application_port);
    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind(address)?;
    let server = startup::run(listener, connection_pool)?;
    server.await
}
