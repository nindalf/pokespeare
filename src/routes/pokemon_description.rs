use actix_http::http;
use actix_web::{get, web, HttpResponse};
use sqlx::PgPool;

use crate::{
    external_api::{
        pokemon::{self, PokeError},
        shakespeare,
    },
    storage::db,
};

#[derive(serde::Serialize)]
struct Response {
    name: String,
    description: String,
}

#[derive(serde::Serialize)]
struct ErrorResponse {
    code: u16,
    message: String,
    error: String,
}
#[derive(thiserror::Error, Debug)]
pub enum ResponseError {
    #[error("Pokemon not found")]
    NotFound,
    #[error("Call to external API `{0}` failed")]
    ExternalAPIError(ExternalAPI),
    #[error("Storage failed")]
    StorageError,
}

#[derive(Debug)]
pub enum ExternalAPI {
    PokeAPI,
    Shakespeare,
}

impl std::fmt::Display for ExternalAPI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExternalAPI::PokeAPI => f.write_str("PokeAPI"),
            ExternalAPI::Shakespeare => f.write_str("Shakespeare"),
        }
    }
}

impl ResponseError {
    pub fn name(&self) -> String {
        match self {
            Self::NotFound => "NotFound".to_string(),
            Self::ExternalAPIError(_) => "ExternalAPIError".to_string(),
            Self::StorageError => "StorageError".to_string(),
        }
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> http::StatusCode {
        match self {
            Self::NotFound => http::StatusCode::NOT_FOUND,
            ResponseError::ExternalAPIError(_) => http::StatusCode::INTERNAL_SERVER_ERROR,
            ResponseError::StorageError => http::StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            code: status_code.as_u16(),
            message: self.to_string(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
    }
}

#[get("/pokemon/{pokemon}")]
pub async fn pokemon_description(
    path: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse, ResponseError> {
    let pokemon_name = path.into_inner();
    let storage_result = db::get_pokemon(pool.get_ref(), &pokemon_name).await;
    if let Ok(record) = storage_result {
        if let Some(description) = record.shakespeare_description {
            return Ok(HttpResponse::Ok()
                .insert_header((http::header::CONTENT_TYPE, "application/json"))
                .json(Response {
                    name: record.name,
                    description,
                }));
        }
    }

    let pokemon = pokemon::get_pokemon_id_and_description(&pokemon_name)
        .await
        .map_err(|err| {
            // TODO log error
            if err == PokeError::NotFound {
                return ResponseError::NotFound;
            }
            ResponseError::ExternalAPIError(ExternalAPI::PokeAPI)
        })?;

    let shakespeare_description =
        shakespeare::translate(&pokemon.description)
            .await
            .map_err(|_|
                // TODO log error
                ResponseError::ExternalAPIError(ExternalAPI::Shakespeare))?;

    db::store_pokemon(
        pool.get_ref(),
        pokemon.id,
        &pokemon.name,
        &pokemon.description,
        &shakespeare_description,
    )
    .await
    .map_err(|_|
        // TODO log error
        ResponseError::StorageError)?;

    return Ok(HttpResponse::Ok()
        .insert_header((http::header::CONTENT_TYPE, "application/json"))
        .json(Response {
            name: pokemon.name,
            description: shakespeare_description,
        }));
}
