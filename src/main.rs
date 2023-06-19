use actix_web::{App, HttpServer};


mod controller {
    pub mod todo_controller;
}
// mod controller::todo_controller;
use controller::todo_controller::configure;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(configure)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
