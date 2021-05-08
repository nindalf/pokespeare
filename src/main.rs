use std::net::TcpListener;

use pokespeare::run;
mod configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    let address = format!("localhost:{}", settings.application_port);
    let listener = TcpListener::bind(address)?;
    let server = run(listener)?;
    server.await
}
