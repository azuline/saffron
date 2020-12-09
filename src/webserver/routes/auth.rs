use crate::config::Config;
use crate::models::{Session, User};
use crate::webserver::responder::Template;
use actix_identity::Identity;
use actix_web::web::{Data, Form};
use actix_web::{get, post, Either, HttpResponse};
use serde::{Deserialize, Serialize};
use tera::Context;

type Response = Either<Template, HttpResponse>;

#[get("/login")]
pub async fn login(id: Identity) -> Response {
    match Session::parse(&id) {
        Some(_) => Either::B(HttpResponse::Found().header("Location", "/").finish()),
        None => Either::A(Template("login.html", Context::new())),
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    token: String,
}

#[post("/login")]
pub async fn take_login(
    id: Identity,
    config: Data<Config>,
    form: Form<LoginForm>,
) -> Response {
    if Session::parse(&id).is_some() {
        return Either::B(HttpResponse::Found().header("Location", "/").finish());
    };

    if let Ok(token) = hex::decode(&form.token) {
        if let Some(user) = User::from_token(&config.db_pool, &token).await {
            // Set session cookie.
            let session = Session::from_user(&user);
            id.remember(serde_json::to_string(&session).unwrap());

            return Either::B(HttpResponse::Found().header("Location", "/").finish());
        }
    }

    let mut context = Context::new();
    context.insert("message", "Invalid authorization token.");

    return Either::A(Template("login.html", context));
}

#[post("/logout")]
pub async fn take_logout(id: Identity) -> HttpResponse {
    id.forget();
    HttpResponse::Found().header("Location", "/login").finish()
}
