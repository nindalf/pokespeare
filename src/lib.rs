mod routes;

use std::net::TcpListener;

use actix_web::{dev::Server, App, HttpServer};

pub fn run(listener: TcpListener) -> std::io::Result<Server> {
    Ok(HttpServer::new(|| {
        App::new()
            .service(routes::pokemon_description)
            .service(routes::health_check)
    })
    .listen(listener)?
    .run())
}
