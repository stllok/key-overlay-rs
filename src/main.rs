use anyhow::Result;

use key_overlay_rs::{app, cli};

fn main() -> Result<()> {
    let args = cli::parse_args();
    app::run(&args.config)
}
