# key-overlay-rs

A lightweight overlay application that visualizes keyboard and mouse input in real-time. Perfect for streamers, content creators, or anyone who wants to show their key presses on screen.

## Features

- Real-time key press visualization
- Mouse button tracking support
- Configurable key colors and sizes
- Smooth animations with fading effects
- Press counter display
- Hot-reloadable configuration
- Cross-platform support (Windows, macOS, Linux)

## Installation

### Prerequisites

- Rust toolchain (1.82 or later)
- Cargo package manager

### From Source

```bash
git clone https://github.com/yourusername/key-overlay-rs.git
cd key-overlay-rs
cargo build --release
```

The compiled binary will be at `target/release/key-overlay-rs` on Windows and Linux, or `target/release/key-overlay-rs.exe` on Windows.

### Building

```bash
# Debug build (faster compilation)
cargo build

# Release build (optimized, smaller binary)
cargo build --release
```

## Usage

Run the application with a configuration file:

```bash
key-overlay --config config.toml
```

On first run, if the config file doesn't exist, it will be created automatically with default values.

## Configuration

The application uses TOML configuration files. See [docs/config.md](docs/config.md) for the complete configuration reference.

## Development

### Build

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Lint

```bash
cargo clippy --all-targets -- -D warnings
```

### Format

```bash
cargo fmt
```

### Documentation

```bash
cargo doc --open
```

## License

MIT License - see LICENSE file for details

## Credits

Built with Rust and [egui](https://github.com/emilk/egui) for rendering, using [rdev](https://github.com/rustdesk-org/rdev) for cross-platform input capture.
