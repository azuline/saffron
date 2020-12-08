use crate::commands::Start;
use crate::routes;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

pub async fn start(opts: Start) -> std::io::Result<()> {
    let bind_addr = format!("{}:{}", &opts.host, &opts.port);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(routes::index)
            .service(routes::login)
    })
    .bind(bind_addr)?
    .run()
    .await
}
