use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/hello1")]
pub async fn hello1() -> impl Responder {
    HttpResponse::Ok().body("Hello world!1")
}

#[get("/hello2")]
pub async fn hello2() -> impl Responder {
    HttpResponse::Ok().body("Hello world!2")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


