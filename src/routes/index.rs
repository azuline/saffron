use actix_web::error::ErrorInternalServerError;
use actix_web::web::Data;
use actix_web::{get, Error, HttpResponse, Responder};
use tera::{Context, Tera};

#[get("/")]
async fn index(tmpl: Data<Tera>) -> Result<HttpResponse, Error> {
    let html = tmpl
        .render("index.html", &Context::new())
        .map_err(|_| ErrorInternalServerError("Template error."))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(html))
}

#[get("/login")]
async fn login() -> impl Responder {
    HttpResponse::Ok().body("Log in?")
}
