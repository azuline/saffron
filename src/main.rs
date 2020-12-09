use clap::Clap;
use dotenv::dotenv;
use saffron::commands::{Command, Commands};
use saffron::config::Config;
use saffron::{manage, webserver};

#[actix_web::main]
async fn main() {
    dotenv().ok();
    sodiumoxide::init().unwrap();

    let commands: Commands = Commands::parse();
    let config: Config = Config::read().await;

    match commands.command {
        Command::Start(opts) => webserver::start(opts, config).await.unwrap(),
        Command::User(subcommand) => manage::run(subcommand, config).await,
    }
}
