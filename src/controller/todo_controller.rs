use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/")
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello)),
    );
}
