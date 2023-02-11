use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/")]
async fn echo(request_body: String) -> impl Responder {
    HttpResponse::Ok().body(request_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Rustacean")
}
