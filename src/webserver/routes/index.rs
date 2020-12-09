use crate::models::Session;
use crate::webserver::responder::Template;
use actix_identity::Identity;
use actix_web::{get, Either, HttpResponse};
use tera::Context;

type Response = Either<Template, HttpResponse>;

#[get("/")]
async fn index(id: Identity) -> Response {
    if let Some(user) = Session::parse(&id) {
        let mut context = Context::new();
        context.insert("user", &user);
        return Either::A(Template("index.html", context));
    };

    Either::B(HttpResponse::Found().header("Location", "/login").finish())
}
