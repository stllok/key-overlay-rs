# input/ - Input Backend Abstraction

## Overview

Trait-based input abstraction with platform implementations and test mock.

## Structure

```
input/
├── mod.rs           # Re-exports
├── backend.rs       # InputBackend trait + MockBackend + create_backend()
├── key_mapping.rs   # Platform key code → string translation
└── rdev_backend.rs  # rdev-based platform implementation
```

## Where to Look

| Task | Location |
|------|----------|
| Add new input method | Implement `InputBackend` trait |
| Mock input in tests | `backend.rs` → `MockBackend` |
| Fix platform-specific capture | `rdev_backend.rs` |
| Change key name display | `key_mapping.rs` |

## Key Types

### InputBackend Trait

```rust
pub trait InputBackend: Send + 'static {
    fn start(&mut self, tx: Sender<InputEvent>) -> Result<(), AppError>;
    fn stop(&mut self) -> Result<(), AppError>;
}
```

- Object-safe (usable as `Box<dyn InputBackend>`)
- Uses `crossbeam_channel::Sender` for event delivery
- Returns `AppError` on failure

### MockBackend (Testing)

```rust
let backend = MockBackend::new(vec![
    InputEvent::KeyPress("A".to_string()),
])
.with_start_error("simulated failure");  // Optional error injection
```

Use for deterministic tests without real device access.

### create_backend() Factory

Returns platform-appropriate backend:
- Windows/macOS/Linux: `RdevBackend`
- Unsupported platforms: `MockBackend`

## Conventions

- Platform code stays in `rdev_backend.rs`
- All events flow through `InputEvent` enum (defined in `types.rs`)
- MockBackend supports error injection for failure path testing
