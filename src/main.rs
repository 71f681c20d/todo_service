
mod controller {
    pub mod todo_controller;
}

use actix_web::{HttpServer, web, App};
use controller::todo_controller;



// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(todo_controller::hello)
//             .service(todo_controller::echo)
//             .route("/hey", web::get().to(todo_controller::manual_hello))
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = configure_server().await;
    bind_server(server).await
}

async fn configure_server() -> actix_web::dev::Server {
    HttpServer::new(|| {
        App::new()
            .service(todo_controller::hello)
            .service(todo_controller::echo)
            .route("/hey", web::get().to(todo_controller::manual_hello))
    })
    .bind(("127.0.0.1", 8080))
    .expect("Failed to bind server")
    .run()
}

async fn bind_server(server: actix_web::dev::Server) -> std::io::Result<()> {
    server.await
}


