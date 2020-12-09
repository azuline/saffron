use clap::Clap;
use dotenv::dotenv;
use fragrance::commands::{Command, Commands, User};
use fragrance::config::Config;
use fragrance::{manage, webserver};

#[actix_web::main]
async fn main() {
    dotenv().ok();
    sodiumoxide::init().unwrap();

    let commands: Commands = Commands::parse();
    let config: Config = Config::read().await;

    match commands.command {
        Command::Start(opts) => webserver::start(opts, config).await.unwrap(),
        Command::User(subcommand) => match subcommand.command {
            User::Create(opts) => manage::create_user(opts, config).await,
            User::Reset(opts) => manage::reset_user(opts, config).await,
            User::List => manage::list_users(config).await,
        },
    }
}
