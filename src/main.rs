
mod controller {
    pub mod todo_controller;
}

use controller::todo_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = todo_controller::configure_server().await;
    bind_server(server).await
}

async fn bind_server(server: actix_web::dev::Server) -> std::io::Result<()> {
    server.await
}
