#[actix_rt::test]
async fn health_check_works() {
    let test_app = crate::spawn::spawn_app().await;
    let endpoint = format!("{}/health_check", test_app.address);
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
