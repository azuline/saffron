use super::routes;
use crate::commands::Start;
use crate::config::Config;
use actix_files as fs;
use actix_identity::{CookieIdentityPolicy, IdentityService};
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

    HttpServer::new(move || {
        let tera = Tera::new(templates_dir).unwrap();
        let upload_dir = config.upload_directory.to_str().unwrap();

        App::new()
            .data(tera)
            .data(config.clone())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&config.secret_key.clone())
                    .name("session")
                    .secure(false)
                    .http_only(true)
                    .same_site(SameSite::Strict),
            ))
            // Always wrap with Logger middleware last.
            .wrap(Logger::default())
            .service(fs::Files::new("/static", &static_dir))
            .service(fs::Files::new("/f", upload_dir))
            .service(routes::index)
            .service(routes::login)
            .service(routes::take_login)
            .service(routes::take_logout)
    })
    .bind(bind_addr)?
    .run()
    .await
}
