use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1")]
pub struct Commands {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Start(Start),
    User(UserCommand),
}

#[derive(Clap, Debug)]
#[clap(about = "Start the webserver")]
pub struct Start {
    #[clap(short, long, default_value = "127.0.0.1")]
    pub host: String,
    #[clap(short, long, default_value = "8000")]
    pub port: u16,
}

#[derive(Clap)]
pub struct UserCommand {
    #[clap(subcommand)]
    pub command: User,
}

#[derive(Clap, Debug)]
#[clap(about = "Users and token management", version = "0.1")]
pub enum User {
    Create(Create),
    Reset(Reset),
    List,
}

#[derive(Clap, Debug)]
#[clap(about = "Create new user")]
pub struct Create {
    pub nickname: String,
}

#[derive(Clap, Debug)]
#[clap(about = "Reset user token")]
pub struct Reset {
    pub user_id: i64,
}
