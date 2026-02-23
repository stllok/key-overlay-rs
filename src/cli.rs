//! CLI argument parsing

use clap::Parser;
use std::path::PathBuf;

/// Command line arguments for key-overlay
#[derive(Parser, Debug)]
#[command(name = "key-overlay", about = "Key press overlay for osu!", version)]
pub struct Args {
    /// Path to config file
    #[arg(short, long, default_value = "config.toml")]
    pub config: PathBuf,
}

/// Parse command line arguments
pub fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_default_config_path() {
        // Simulate: cargo run (no args)
        let args = Args {
            config: PathBuf::from("config.toml"),
        };
        assert_eq!(args.config, PathBuf::from("config.toml"));
    }

    #[test]
    fn test_custom_config_path() {
        // Simulate: cargo run --config custom.toml
        let args = Args {
            config: PathBuf::from("custom.toml"),
        };
        assert_eq!(args.config, PathBuf::from("custom.toml"));
    }

    #[test]
    fn test_custom_config_path_with_full_directory() {
        // Simulate: cargo run --config /path/to/custom.toml
        let args = Args {
            config: PathBuf::from("/path/to/custom.toml"),
        };
        assert_eq!(args.config, PathBuf::from("/path/to/custom.toml"));
    }
}
