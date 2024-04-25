use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short)]
    pub port: u16
}
