use crate::config::Config;
use crate::models::{File, Session};
use crate::webserver::responder::Template;
use actix_identity::Identity;
use actix_web::web::Data;
use actix_web::{get, web, Either, HttpResponse};
use serde::Deserialize;
use tera::Context;

#[derive(Deserialize)]
struct Info {
    page: Option<i64>,
}

type Response = Either<Template, HttpResponse>;

const PER_PAGE: i64 = 50;

#[get("/")]
async fn index(info: web::Query<Info>, id: Identity, config: Data<Config>) -> Response {
    let session = match Session::parse(&id) {
        Some(user) => user,
        None => {
            return Either::B(
                HttpResponse::Found().header("Location", "/login").finish(),
            )
        }
    };

    let page = info.page.unwrap_or(1);

    let files =
        match File::all_of_user(&config.db_pool, session.user_id, page, PER_PAGE).await
        {
            Ok(files) => files,
            _ => return Either::B(HttpResponse::InternalServerError().finish()),
        };

    let num_pages = match File::count(&config.db_pool, session.user_id).await {
        Ok(x) => (x - 1) / PER_PAGE + 1,
        _ => return Either::B(HttpResponse::InternalServerError().finish()),
    };

    let mut context = Context::new();
    context.insert("user", &session);
    context.insert("files", &files);
    context.insert("cur_page", &page);
    context.insert("num_pages", &num_pages);

    Either::A(Template("index.html", context))
}
