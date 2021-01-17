use crate::config::Config;
use crate::models::{File, Session};
use crate::webserver::responder::Template;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{get, Either, HttpResponse};
use tera::Context;

type Response = Either<Template, HttpResponse>;

#[get("/")]
async fn index(id: Identity, config: Data<Config>) -> Response {
    let session = match Session::parse(&id) {
        Some(user) => user,
        None => {
            return Either::B(
                HttpResponse::Found().header("Location", "/login").finish(),
            )
        }
    };

    let files = match File::all_of_user(&config.db_pool, session.user_id).await {
        Ok(files) => files,
        _ => return Either::B(HttpResponse::InternalServerError().finish()),
    };

    let mut context = Context::new();
    context.insert("user", &session);
    context.insert("files", &files);

    Either::A(Template("index.html", context))
}
