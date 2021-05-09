use std::net::TcpListener;

use pokespeare::configuration::Settings;
use sqlx::prelude::Connection;
use sqlx::prelude::Executor;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let mut settings =
        pokespeare::configuration::get_configuration().expect("Failed to read configuration.");
    settings.database.database_name = Uuid::new_v4().to_string();
    let db_pool = configure_database(settings).await;
    let listener = TcpListener::bind("localhost:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = pokespeare::startup::run(listener, db_pool.clone()).unwrap();
    tokio::spawn(server);
    let address = format!("http://localhost:{}", port);
    TestApp { address, db_pool }
}

async fn configure_database(settings: Settings) -> PgPool {
    // Create database
    let mut connection = PgConnection::connect(&settings.database.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres");
    connection
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            settings.database.database_name
        ))
        .await
        .expect("Failed to create database.");
    // Migrate database
    let connection_pool = PgPool::connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
    connection_pool
}
