use actix_web::{get, web, Responder};

#[get("/pokemon/{pokemon}")]
pub async fn pokemon_description(path: web::Path<String>) -> impl Responder {
    let pokemon = path.into_inner();
    format!("Hello {}!", &pokemon)
}
