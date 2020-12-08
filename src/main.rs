use clap::Clap;
use fragrance::commands::{Command, Commands};
use fragrance::webserver;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let commands: Commands = Commands::parse();

    match commands.command {
        Command::Start(opts) => webserver::start(opts).await,
    }
}
