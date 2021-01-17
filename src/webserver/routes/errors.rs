use crate::webserver::responder::Template;
use actix_web::{get, HttpResponse};
use tera::Context;

#[get("/404")]
pub async fn error_404() -> Template {
    let mut context = Context::new();
    context.insert("message", "you are lost");

    Template("error.html", context)
}

pub async fn redirect_404() -> HttpResponse {
    HttpResponse::Found().header("Location", "/404").finish()
}
