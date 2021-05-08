use std::net::TcpListener;

use actix_web::{dev::Server, web::Data, App, HttpServer};
use sqlx::PgPool;

use crate::routes;

pub fn run(listener: TcpListener, db_pool: PgPool) -> std::io::Result<Server> {
    // Wrap the pool using web::Data, which boils down to an Arc smart pointer
    let db_pool = Data::new(db_pool);

    Ok(HttpServer::new(move || {
        App::new()
            .service(routes::pokemon_description)
            .service(routes::health_check)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run())
}
