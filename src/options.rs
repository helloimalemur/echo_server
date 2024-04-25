use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    #[arg(short)]
    port: u64
}
