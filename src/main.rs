use std::net::TcpListener;

use pokespeare::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:3000")?;
    let server = run(listener)?;
    server.await
}
