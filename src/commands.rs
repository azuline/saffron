use clap::Clap;

#[derive(Clap)]
#[clap(version = "0.1", author = "blissful <blissful@sunsetglow.net>")]
pub struct Commands {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Clap)]
pub enum Command {
    Start(Start),
}

#[derive(Clap, Debug)]
#[clap(about = "Start the webserver")]
pub struct Start {
    #[clap(short, long, default_value = "127.0.0.1")]
    pub host: String,
    #[clap(short, long, default_value = "8000")]
    pub port: u16,
}
