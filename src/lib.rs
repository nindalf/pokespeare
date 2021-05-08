use std::net::TcpListener;

use actix_web::{dev::Server, get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/pokemon/{pokemon}")]
async fn pokemon_description(path: web::Path<String>) -> impl Responder {
    let pokemon = path.into_inner();
    format!("Hello {}!", &pokemon)
}

#[get("/health_check")]
async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        App::new()
            .service(pokemon_description)
            .service(health_check)
    })
    .listen(listener)?
    .run())
}
