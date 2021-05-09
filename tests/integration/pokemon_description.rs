#[derive(serde::Deserialize)]
struct Response {
    name: String,
    description: String,
}

#[derive(serde::Deserialize, Debug)]
struct ErrorResponse {
    code: u16,
    message: String,
    error: String,
}

#[actix_rt::test]
async fn pokemon_fetch_works() {
    let test_app = crate::spawn::spawn_app().await;
    let endpoint = format!("{}/pokemon/charizard", test_app.address);
    let client = reqwest::Client::new();
    let response = client
        .get(endpoint)
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());

    let response = response
        .json::<Response>()
        .await
        .expect("Failed to parse json");
    assert_eq!(response.name, "charizard");
    assert_eq!(
        response.description,
        "Spits fire yond is hot enow to melt boulders. Known to cause forest fires unintentionally."
    );
}
