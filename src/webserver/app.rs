use super::routes;
use crate::commands::Start;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{cookie::SameSite, middleware::Logger};
use actix_web::{App, HttpServer};
use env_logger::Env;
use tera::Tera;

pub async fn start(opts: Start) -> std::io::Result<()> {
    let bind_addr = format!("{}:{}", &opts.host, &opts.port);
    let templates_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/views/templates/**/*.html");
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/views/static");
    let secret_key: [u8; 32] = [0; 32];

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        let tera = Tera::new(&templates_dir).unwrap();

        App::new()
            .data(tera)
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(&secret_key)
                    .secure(false)
                    .http_only(true)
                    .same_site(SameSite::Strict),
            )
            .service(fs::Files::new("/static", &static_dir))
            .service(routes::index)
            .service(routes::login)
    })
    .bind(bind_addr)?
    .run()
    .await
}
