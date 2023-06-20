use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello1")]
async fn hello1() -> impl Responder {
    HttpResponse::Ok().body("Hello world!1")
}

#[get("/hello2")]
async fn hello2() -> impl Responder {
    HttpResponse::Ok().body("Hello world!2")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub async fn configure_server() -> actix_web::dev::Server {
  HttpServer::new(|| {
      App::new()
          // Declare all your endpoint routes to be served
          .service(hello)
          .service(hello1)
          .service(hello2)
          .service(echo)
          .route("/hey", web::get().to(manual_hello))
  })
  .bind(("127.0.0.1", 8080))
  .expect("Failed to bind server")
  .run()
}

