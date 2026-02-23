# AGENTS.md - key-overlay-rs

> Guidelines for agentic coding agents operating in this Rust repository.

## Project Overview

- **Language**: Rust (Edition 2024)
- **Type**: Native keyboard overlay application
- **Package Manager**: Cargo
- **Stack**: egui + egui_overlay for UI, rdev for input capture

---

## Build, Lint, and Test Commands

### Build

```bash
# Debug build (default)
cargo build

# Release build (optimized)
cargo build --release
```

### Run

```bash
# Run the application
cargo run

# Run with release optimizations
cargo run --release
```

### Test

```bash
# Run all tests
cargo test

# Run a single test by name (substring match)
cargo test test_name

# Run a single test in a specific module
cargo test module_name::test_name

# Run tests with output shown
cargo test -- --nocapture

# Run a specific test file (by test name pattern)
cargo test --test integration_test_name

# Run ignored tests
cargo test -- --ignored
```

### Lint & Format

```bash
# Check for compilation errors without building
cargo check

# Run Clippy (linter) - all targets including tests
cargo clippy --all-targets -- -D warnings

# Format code (uses rustfmt defaults)
cargo fmt

# Check formatting without applying changes
cargo fmt -- --check
```

### Documentation

```bash
# Generate and open documentation
cargo doc --open
```

---

## Structure

```
src/
├── main.rs              # Binary entry point (CLI + app bootstrap)
├── lib.rs               # Library root - declares 14 public modules
├── app.rs               # Main application state & lifecycle
├── bars.rs              # Key bar visualization
├── cli.rs               # CLI argument parsing (clap)
├── color.rs             # Color handling & parsing
├── config.rs            # Configuration loading (TOML)
├── fading.rs            # Fade animation logic
├── font.rs              # Font loading
├── key_map.rs           # Key name mapping
├── layout.rs            # Overlay layout calculations
├── logging.rs           # Tracing setup
├── renderer.rs          # egui rendering
├── types.rs             # Core types (InputEvent, AppError)
├── watcher.rs           # Config file watcher
└── input/               # Input backend abstraction (see input/AGENTS.md)
    ├── mod.rs
    ├── backend.rs       # InputBackend trait + MockBackend
    ├── key_mapping.rs   # Platform key code translation
    └── rdev_backend.rs  # rdev-based platform implementation

tests/
└── integration.rs       # Integration tests

config.toml              # Runtime configuration (user-editable)
```

---

## Where to Look

| Task | Location | Notes |
|------|----------|-------|
| Add new CLI flag | `src/cli.rs` | Uses clap derive macros |
| Modify overlay appearance | `src/renderer.rs`, `src/bars.rs` | egui painting |
| Change fade behavior | `src/fading.rs` | Fade animation logic |
| Add config option | `src/config.rs` + `config.toml` | TOML-based, hot-reload via watcher |
| Handle new input type | `src/input/` | Add to InputBackend impl |
| Add new error variant | `src/types.rs` | AppError enum |
| Adjust key display | `src/key_map.rs` | Key name mappings |
| Platform-specific input | `src/input/rdev_backend.rs` | rdev integration |

---

## Code Style Guidelines

### Imports

```rust
// Group imports in this order, separated by blank lines:
// 1. Standard library
// 2. External crates (alphabetically)
// 3. Current crate modules (alphabetically)
// 4. Parent/sibling modules (use super, crate, self)

use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::config::Settings;
use crate::error::AppError;
use super::handler;
```

### Formatting

- Use `cargo fmt` before committing. Do not argue with rustfmt.
- Maximum line width: 100 characters (default)
- Use 4 spaces for indentation (no tabs)
- Opening braces on the same line

### Naming Conventions

| Item | Convention | Example |
|------|------------|---------|
| Crates | `snake_case` | `key_overlay_rs` |
| Modules | `snake_case` | `mod key_handler;` |
| Types (struct/enum) | `PascalCase` | `struct KeyEvent;` |
| Traits | `PascalCase` | `trait Drawable;` |
| Functions | `snake_case` | `fn process_input()` |
| Methods | `snake_case` | `impl.count_keys()` |
| Local variables | `snake_case` | `let key_count = 0;` |
| Constants | `SCREAMING_SNAKE_CASE` | `const MAX_KEYS: usize = 100;` |
| Statics | `SCREAMING_SNAKE_CASE` | `static VERSION: &str = "0.1.0";` |
| Type parameters | Single uppercase letter or PascalCase | `T`, `K`, `KeyHandler` |
| Lifetimes | Short lowercase | `'a`, `'static` |

### Types

- Prefer strong typing over primitives. Create newtypes for domain concepts:

```rust
// Good: Domain-specific types
#[derive(Debug, Clone)]
pub struct KeyCode(u32);

#[derive(Debug, Clone)]
pub struct OverlayPosition { x: f32, y: f32 }

// Avoid: Raw primitives everywhere
fn handle_key(code: u32) { } // What's a valid code?
```

- Use `#[derive]` attributes for common traits when appropriate:

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    pub opacity: f32,
}
```

- Prefer `enum` over `bool` for multi-state values:

```rust
// Prefer
enum OverlayState { Hidden, Visible, Fading }

// Over
struct Overlay { is_visible: bool, is_fading: bool }
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Propagate errors with `?` operator
- Use `thiserror` or `anyhow` for custom error types (when dependencies added)
- Never use `unwrap()` or `expect()` in production code paths; use `?` or proper error handling

```rust
// Good: Proper error propagation
fn load_config(path: &Path) -> Result<Config, io::Error> {
    let contents = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

// Acceptable: Known-safe cases in tests or assertions
#[test]
fn test_parse() {
    let value: i32 = "42".parse().unwrap(); // OK in tests
}
```

- Use `Option<T>` for values that may be absent
- Prefer `ok_or()` or `ok_or_else()` to convert Option to Result

### Documentation

- Document all public items with `///` doc comments
- Include examples in documentation when helpful

```rust
/// Represents a keyboard event captured by the overlay.
///
/// # Example
///
/// ```
/// let event = KeyEvent::new(KeyCode::from(65), KeyState::Pressed);
/// assert!(event.is_pressed());
/// ```
pub struct KeyEvent {
    code: KeyCode,
    state: KeyState,
}
```

---

## Testing Guidelines

### Test Organization

- Unit tests: `#[cfg(test)]` modules within source files (config.rs, types.rs, bars.rs)
- Integration tests: `tests/integration.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event_creation() {
        // Arrange-Act-Assert pattern
    }
}
```

### Test Naming

- Format: `test_<function>_<scenario>_<expected_result>`
- Example: `test_parse_config_missing_file_returns_error`

---

## Clippy Lints

Run `cargo clippy` frequently. Treat warnings as errors:

```bash
cargo clippy --all-targets -- -D warnings
```

Critical lints:
- `clippy::unwrap_used` - Avoid unwrap in production code
- `clippy::expect_used` - Avoid expect in production code
- `clippy::panic` - Avoid explicit panic

---

## Anti-Patterns (THIS PROJECT)

1. **Never use `unwrap()`/`expect()` in production code** - Use `?` or proper error handling
2. **Always run `cargo fmt` before committing** - Do not argue with rustfmt
3. **Never skip Clippy warnings** - All warnings are errors (`-D warnings`)

---

## Notes

### Linux System Dependencies

Building on Linux requires system libraries:

```bash
# Ubuntu/Debian
sudo apt install libudev-dev libx11-dev libxtst-dev libevdev-dev

# Fedora
sudo dnf install systemd-devel libX11-devel libXtst-devel libevdev-devel
```

### Git Dependencies

- `rdev` is sourced from git (`https://github.com/rustdesk-org/rdev.git`), not crates.io
- This may affect offline builds and dependency caching

### Config Hot-Reload

- `config.toml` is watched at runtime via `notify-debouncer-full`
- Changes apply without restart

---

## Git Conventions

- Write clear, descriptive commit messages
- Keep commits atomic (one logical change per commit)
- Run `cargo fmt` and `cargo clippy` before committing

---

## Dependencies

When adding dependencies:
1. Consider compile time and binary size impact
2. Prefer well-maintained crates with active development
3. Check for `#[no_std]` compatibility if relevant

Add with: `cargo add <crate-name>`

---

## Quick Reference

| Task | Command |
|------|---------|
| Build | `cargo build` |
| Run | `cargo run` |
| Test all | `cargo test` |
| Test single | `cargo test <name>` |
| Lint | `cargo clippy -- -D warnings` |
| Format | `cargo fmt` |
| Check | `cargo check` |
