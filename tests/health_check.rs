use std::net::TcpListener;

use sqlx::PgPool;

#[actix_rt::test]
async fn health_check_works() {
    let address = spawn_app();
    let endpoint = format!("{}/health_check", address);
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let settings =
        pokespeare::configuration::get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect_lazy(&settings.database.connection_string())
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind("localhost:0").unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = pokespeare::startup::run(listener, connection_pool).unwrap();
    tokio::spawn(server);
    format!("http://localhost:{}", port)
}
