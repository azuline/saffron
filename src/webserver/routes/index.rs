use actix_web::{get, HttpResponse, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Log in?")
}
