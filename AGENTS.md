# AGENTS.md - key-overlay-rs

> Guidelines for agentic coding agents operating in this Rust repository.

## Project Overview

- **Language**: Rust (Edition 2024)
- **Type**: Native application
- **Package Manager**: Cargo

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

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_event_creation() {
        // Arrange
        let code = KeyCode::from(65);
        
        // Act
        let event = KeyEvent::new(code, KeyState::Pressed);
        
        // Assert
        assert!(event.is_pressed());
    }
}
```

### Test Naming

- Test names should describe what is being tested: `test_<function>_<scenario>_<expected_result>`
- Example: `test_parse_config_missing_file_returns_error`

### Test Structure

Follow Arrange-Act-Assert pattern. Keep tests focused on one behavior each.

---

## Module Structure

```
src/
├── main.rs          # Binary entry point
├── lib.rs           # Library root (if applicable)
├── config.rs        # Configuration handling
├── error.rs         # Error types
├── key_handler.rs   # Keyboard event handling
└── overlay/
    ├── mod.rs       # Module declaration
    ├── renderer.rs  # Rendering logic
    └── window.rs    # Window management
```

---

## Clippy Lints

Run `cargo clippy` frequently. Treat warnings as errors:

```bash
cargo clippy --all-targets -- -D warnings
```

Common lints to respect:
- `clippy::unwrap_used` - Avoid unwrap in production code
- `clippy::expect_used` - Avoid expect in production code
- `clippy::panic` - Avoid explicit panic

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
