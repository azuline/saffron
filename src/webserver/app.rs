use super::routes;
use crate::commands::Start;
use crate::config::Config;
use actix_files as fs;
use actix_session::CookieSession;
use actix_web::{cookie::SameSite, middleware::Logger};
use actix_web::{App, HttpServer};
use env_logger::Env;
use tera::Tera;

pub async fn start(opts: Start, config: Config) -> std::io::Result<()> {
    let bind_addr = format!("{}:{}", &opts.host, &opts.port);
    let templates_dir =
        concat!(env!("CARGO_MANIFEST_DIR"), "/views/templates/**/*.html");
    let static_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/views/static");

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    dbg!(&config);

    HttpServer::new(move || {
        let tera = Tera::new(&templates_dir).unwrap();

        App::new()
            .data(tera)
            .wrap(Logger::default())
            .wrap(
                CookieSession::signed(&config.secret_key)
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
