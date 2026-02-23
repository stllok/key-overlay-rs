//! CLI argument parsing

use clap::Parser;

/// Command line arguments
#[derive(Parser, Debug)]
pub struct Args {
    /// Config file path
    #[arg(short, long)]
    pub config: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}
