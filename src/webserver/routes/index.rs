use crate::webserver::responder::Template;
use actix_web::{get, Responder};
use tera::Context;

#[get("/")]
async fn index() -> Template {
    Template("index.html", Context::new())
}

#[get("/login")]
async fn login() -> impl Responder {
    Template("login.html", Context::new())
}
