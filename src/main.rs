use clap::Clap;
use dotenv::dotenv;
use fragrance::commands::{Command, Commands};
use fragrance::config::Config;
use fragrance::webserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    sodiumoxide::init().unwrap();

    let commands: Commands = Commands::parse();
    let config: Config = Config::read().await;

    match commands.command {
        Command::Start(opts) => webserver::start(opts, config).await,
    }
}
