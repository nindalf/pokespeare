use reqwest::StatusCode;
#[derive(serde::Deserialize)]
struct ApiResponse {
    success: SuccessFlag,
    contents: Contents,
}

#[derive(serde::Deserialize)]
struct Contents {
    translated: String,
}

#[derive(serde::Deserialize)]
struct SuccessFlag {
    total: i32,
}
#[derive(thiserror::Error, Debug, PartialEq)]
pub(crate) enum ShakespeareError {
    #[error("Not Found")]
    TranslationFailed,
    #[error("APIError")]
    APIError,
}

// TODO change this fn to return thiserror instead of anyhow
// Check for specific errors in unit tests
pub(crate) async fn translate(input: &str) -> Result<String, ShakespeareError> {
    #[cfg(not(test))]
    let url = "https://api.funtranslations.com/translate/shakespeare.json";
    #[cfg(test)]
    let url = &mockito::server_url();

    let client = reqwest::Client::new();
    let form_data = [("text", input)];
    let resp = client
        .post(url)
        .form(&form_data)
        .send()
        .await
        .map_err(|_| ShakespeareError::APIError)?;

    if resp.status() != StatusCode::OK {
        return Err(ShakespeareError::TranslationFailed);
    }

    let api_response = resp
        .json::<ApiResponse>()
        .await
        .map_err(|_| ShakespeareError::APIError)?;

    if api_response.success.total <= 0 {
        return Err(ShakespeareError::TranslationFailed);
    }

    Ok(api_response.contents.translated)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[actix_rt::test]
    async fn test_success_response() {
        let _m = mockito::mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body_from_file("./src/external_api/test_responses/shakespeare_success.json")
            .create();
        match translate(
            "You gave Mr. Tim a hearty meal, but unfortunately what he ate made him die.",
        )
        .await
        {
            Ok(translation) => {
                assert_eq!(translation, "Thee did giveth mr. Tim a hearty meal,  but unfortunately what he did doth englut did maketh him kicketh the bucket.");
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }

    #[actix_rt::test]
    async fn test_bad_input() {
        let _m = mockito::mock("POST", "/")
            .with_status(404)
            .with_header("content-type", "text/plain; charset=utf-8")
            .with_body("Not Found")
            .create();
        let result = translate("bad input").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ShakespeareError::TranslationFailed);
    }

    #[actix_rt::test]
    async fn test_malformed_output() {
        let _m = mockito::mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body(r#"{"success": {"total": 1}}"#)
            .create();
        let result = translate("malformed output").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ShakespeareError::APIError);
    }

    #[actix_rt::test]
    async fn test_bad_translation() {
        let _m = mockito::mock("POST", "/")
            .with_status(200)
            .with_header("content-type", "application/json; charset=utf-8")
            .with_body_from_file("./src/external_api/test_responses/shakespeare_failure.json")
            .create();
        let result = translate("bad translation").await;
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), ShakespeareError::TranslationFailed);
    }
}
