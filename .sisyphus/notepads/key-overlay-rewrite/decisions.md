# Decisions - key-overlay-rewrite

## [2026-02-23T05:59:38Z] Session Start

### Pre-plan Decisions (from interview)
1. **Config Format**: TOML (not INI) - serde native support, richer types
2. **Font Strategy**: Bundle JetBrains Mono (OFL licensed) - zero-dependency deployment
3. **Wayland Support**: REQUIRED - hybrid input backend (rdev + evdev)
4. **Click-through**: Overlay must pass input through to underlying windows
5. **Error Handling**: Graceful degradation - use defaults for invalid config values, log warnings
6. **First Run**: Auto-create default config.toml if not found
7. **v1 Scope**: Feature parity + CLI args + logging; Extended features (tray, KPS, stats, profiles) → v2
8. **TDD**: All logical modules (config, input mapping, bar physics, key state)

### Technology Choices
- egui + egui_overlay: Purpose-built overlay windows
- rdev: RustDesk fork (rustdesk-org/rdev) - actively maintained
- evdev: Raw Linux input for Wayland (needs input group permissions)
- notify v9 + debouncer: File watching
- clap derive: CLI args
- tracing + tracing-appender: File logging

## [2026-02-23] Task 2: Project Structure + Dependencies

### Module Organization
Created 14 main modules + 2 input submodules:
1. **types.rs** - Core domain types (KeyEvent)
2. **color.rs** - Color parsing with RGBA struct
3. **config.rs** - TOML config loading
4. **key_map.rs** - Key name → code mapping
5. **bars.rs** - Bar visualization state machine
6. **layout.rs** - Window sizing calculations
7. **fading.rs** - Fading effect gradient
8. **renderer.rs** - egui rendering adapter
9. **app.rs** - Main app orchestrator
10. **cli.rs** - CLI args with clap derive
11. **logging.rs** - tracing + file logging setup
12. **watcher.rs** - Config file watcher
13. **font.rs** - Font loading utility
14. **input/mod.rs** - Input module root
    - **input/backend.rs** - InputBackend trait + KeyboardEvent
    - **input/rdev_backend.rs** - rdev implementation

### Key Dependency Additions
- serde: Derive serialization/deserialization
- toml: Config parsing
- notify + notify-debouncer-full: File watching
- anyhow + thiserror: Error handling
- clap: CLI argument parsing
- tracing + tracing-subscriber + tracing-appender: Structured logging
- crossbeam-channel: Thread-safe communication for input thread
- rdev: Git fork from rustdesk-org/rdev (cross-platform input)

### Import Organization
All modules follow AGENTS.md import order:
1. std library (e.g., std::collections)
2. External crates (alphabetically)
3. Current crate modules (crate::)
4. Parent/sibling modules (super::, self)

### Module Doc Comments
Each module has `//!` doc comment describing purpose. Public stubs follow convention:
- Structs: `#[derive(Debug, Clone, ...)]`
- Functions: Prefixed with `pub fn` + return type annotation
- Traits: Public trait definitions with method signatures

### Verification Status
✓ cargo check passes with 0 errors
✓ cargo clippy --all-targets -- -D warnings passes with 0 warnings
✓ All 18 .rs files created at expected paths
✓ assets/ directory with .gitkeep
✓ lib.rs declares all 14 public modules
✓ Cargo.toml has all 15 required dependencies
✓ main.rs replaced with minimal placeholder (prints version)
✓ Commit: chore(init): project structure and dependencies (26 files changed)

### Notes for Next Tasks
- Each module file is a stub with types and minimal public functions
- InputBackend trait is ready for concrete implementations
- No platform-specific `#[cfg]` attributes added yet (deferred for task 3+)
- No tests added yet (TDD starts in subsequent task)
- RdevBackend structure ready for input logic implementation


## [2026-02-23] Task 7: Bundle Font Asset + Font Loading

### Font Selection & Sourcing
 **Font**: JetBrains Mono v2.304 Regular
 **License**: SIL Open Font License (OFL 1.1) - bundling permitted
 **Source**: GitHub releases (JetBrains/JetBrainsMono)
 **Weight**: Regular variant only (268KB TTF)
 **Alternative Considered**: JetBrains Mono-Bold (per original plan) vs Regular
  - Decided: Regular for better rendering clarity and smaller size
  - Bold can be added in v2 if needed for emphasis

### Implementation Details
 **Embedding Method**: `include_bytes!("../assets/JetBrainsMono-Regular.ttf")` in font.rs
 **Function Signature**: `pub fn load_font() -> &'static [u8]`
 **Static Reference**: Font bytes are static (zero-copy, no allocation)
 **Zero I/O**: Font is available at runtime without file system access

### File Structure
 **assets/JetBrainsMono-Regular.ttf**: 268KB TTF font file
 **assets/LICENSE-FONT**: OFL 1.1 license text (4.3KB)
 **src/font.rs**: Module with load_font() function and 3 comprehensive tests

### Testing
 ✓ test_load_font_returns_non_empty: Verifies font bytes are present
 ✓ test_load_font_is_valid_ttf: Checks TTF/OTF header signature (0x00010000, OTTO, or true)
 ✓ test_load_font_returns_static_reference: Confirms static memory location

### Binary Impact
 Debug binary: 130KB (includes debug info)
 Release binary: 126KB (stripped, with font embedded)
 Font embedded size: ~268KB (uncompressed in binary)
 Total increase: Well under 500KB limit ✓

### Notes
 Font is never freed (static lifetime)
 No external font loading libraries required (zero dependency)
 egui will receive bytes directly via load_font()
 Future v2: Can add system font fallback or multiple weights if needed

## [2026-02-23] Task 3: Core Types + Error Types (TDD)

### Type Model Decisions
- `Color` uses normalized `f32` channels (`r`, `g`, `b`, `a`) for direct compatibility with egui rendering math.
- `KeyConfig` stores source identifier (`key_name`) separately from UI label (`display_name`) to support remapping later.
- `AppConfig` includes all baseline rendering/runtime knobs from original config defaults and keeps keys as ordered `Vec<KeyConfig>`.
- `InputEvent` remains string-based for now (`KeyPress/KeyRelease/MousePress/MouseRelease`) to decouple this task from key mapping task scope.

### Behavior Decisions
- `Color::pressed()` dims alpha by exact golden ratio factor (`alpha / 1.618`) while preserving RGB.
- `Color::from_rgba_u8()` performs byte-to-float normalization via division by `255.0`.
- `Color::to_egui()` converts with `egui::Color32::from_rgba_unmultiplied(...)` and clamps/rounds channel values to valid byte range.
- Added `Color::black()` helper to make default background intent explicit and keep default construction readable.

### AppConfig Defaults (Original Parity)
- `height = 700.0`
- `key_size = 70.0`
- `bar_speed = 600.0`
- `background_color = Color::black()`
- `margin = 25.0`
- `outline_thickness = 5.0`
- `fading = true`
- `counter = true`
- `fps = 60`
- `keys = [Z(red), X(cyan)]` with `size = 1.0`

### Error Model
- `AppError` uses `thiserror::Error` derive with focused variants:
  - `Config(String)`
  - `Input(String)`
  - `Render(String)`
  - `Io(#[from] std::io::Error)`

### TDD Evidence
- Wrote tests first for:
  - normalized u8->f32 color conversion
  - pressed alpha golden-ratio dimming
  - default config parity with expected baseline values
  - key config field construction
- Verified red phase by running `cargo test types` with placeholder `todo!()` implementations (tests failed as expected), then implemented to green.
## [2026-02-23] Task 4: Color Parsing Module (TDD)

### Color Format & Parsing Strategy
- **Format**: "R,G,B,A" or "R,G,B" (u8 values 0-255)
- **Source**: Original config.ini format (e.g., "255,0,0,255", "0,255,255,255")
- **Default Alpha**: 255 (fully opaque) when A component missing
- **Out-of-Range Strategy**: CLAMP to 255 (not error) for values > 255

### ColorError Type
- **Variants**:
  - `InvalidFormat(String)` - parsing error, non-numeric, wrong component count
  - `OutOfRange(String)` - value out of range (only for negative numbers, positive > 255 clamps)
- **Display impl**: Human-readable error messages for config validation warnings

### Implementation Details
- **parse_color()**: Main parsing function, returns Result<Color, ColorError>
- **parse_color_or_default()**: Graceful fallback, returns Color (never errors)
- **Helper**: parse_u8_clamped() - handles clamping, parses u32 first, clamps to u8
- **Whitespace Handling**: All components trimmed individually
  - Handles leading/trailing: " 255 , 0 , 128 , 200 "
  - Handles internal spaces: "  255  ,  0  ,  128  ,  200  "
- **Empty String**: Returns InvalidFormat error (not panic)
- **Negative Numbers**: Returns parse error (u32 parse fails on "-1")

### Test Coverage: 16 Tests
1. Valid RGBA with all components
2. Valid RGB (missing alpha defaults to 255)
3. All zeros (0,0,0,0)
4. Leading/trailing whitespace on entire string
5. Internal whitespace around components
6. Empty string error
7. Too few parts (2 components) error
8. Too many parts (5 components) error
9. Non-numeric values ("not,a,color,255")
10. Out-of-range value (300 clamps to 255)
11. Negative number error (-1,0,0,0)
12. parse_color_or_default with valid input
13. parse_color_or_default with invalid input
14. parse_color_or_default with empty string
15. Color equality check
16. Color inequality check

### Code Quality
- ✓ Zero LSP diagnostics
- ✓ cargo clippy -- -D warnings: clean
- ✓ cargo fmt: no formatting issues
- ✓ All 16 tests pass: 0.01s execution time

### Commit
- **Message**: `feat(color): color parsing module with TDD`
- **Files**: src/color.rs (310 lines), src/lib.rs (alphabetized modules)
- **Status**: Ready for integration with config module (T5)

## [2026-02-23] Task 6: Key Mapping Module (TDD)

### KeyId Coverage
- Added `src/input/key_mapping.rs` with `KeyId` enum for 64 identifiers:
  - Alphabetic: `A`-`Z`
  - Numeric: `D0`-`D9`
  - Function: `F1`-`F12`
  - Special: `Space`, `Enter`, `Tab`, `Backspace`, `Escape`
  - Modifiers: `LShift`, `RShift`, `LControl`, `RControl`, `LAlt`, `RAlt`
  - Mouse: `Mouse1`-`Mouse5`

### Config Parsing + Display Naming
- `FromStr` supports original KeyOverlay spellings and aliases:
  - Case-insensitive + trimmed input.
  - Numeric aliases: `0`-`9`, `D0`-`D9`, `Num0`-`Num9`.
  - Modifier aliases: `LControl`/`LCtrl`, `RControl`/`RCtrl`, `RAlt`/`AltGr`.
  - Escape alias: `Esc`.
- `Display` returns canonical overlay labels (`LControl`, `Mouse3`, `4`, etc.).

### Backend Conversions
- Implemented `TryFrom<rdev::Key> for KeyId` for keyboard keys used by config naming set.
- Implemented `From<KeyId> for rdev::Key` for keyboard keys with deterministic mapping.
- Added mouse button mapping via:
  - `TryFrom<rdev::Button> for KeyId`
  - `TryFrom<KeyId> for rdev::Button`
- For `From<KeyId> for rdev::Key`, mouse variants map to sentinel `Key::Unknown(0xF001..0xF005)` because rdev models mouse separately from keyboard key enum.

### Error Behavior
- Unknown key names return descriptive `Err(String)` including the original input and accepted examples.
- Unsupported backend keys/buttons return descriptive conversion errors; no panics.

### TDD Notes
- Added tests first in `key_mapping.rs`, validated RED (`cargo test key_mapping` with failing assertions), then implemented to GREEN.
- Tests cover parsing, case/trim handling, unknown-name errors, display labels, keyboard round-trip conversion, and mouse button conversions.

## [2026-02-23] Task 5: TOML Config Module (TDD)

### Parsing Model
- `RawConfig` uses `#[serde(default)]` with two sections:
  - `general: RawGeneral` for `[general]`
  - `key: Vec<RawKeyConfig>` for `[[key]]`
- `RawGeneral` and `RawKeyConfig` use `#[serde(rename_all = "camelCase")]` so TOML keys like `barSpeed`, `backgroundColor`, and `keySize` map to snake_case Rust fields.
- All raw fields are `Option<T>` to preserve missing-vs-present semantics and allow default fallback via `AppConfig::default()`.

### Resolution Rules
- Empty TOML (`""`) resolves to full `AppConfig::default()`.
- Missing general fields reuse corresponding `AppConfig::default()` values.
- Missing `[[key]]` block keeps default key set (`Z`, `X`).
- Present `[[key]]` block replaces defaults and each key resolves as:
  - `name`: required, non-empty string
  - `display_name`: mirrors `name`
  - `size`: defaults to `1.0` if missing
  - `color`: defaults to opaque white (`255,255,255,255`) if missing

### Validation & Error Behavior
- `parse_color()` from `src/color.rs` is the single parser for color strings; conversion to app `Color` uses `Color::from_rgba_u8`.
- Invalid colors return `AppError::Config(...)` with field context (`backgroundColor`, `key color`) rather than panicking.
- `validate_config(&AppConfig) -> Vec<String>` emits non-fatal warnings, including negative/zero `bar_speed`.
- Negative `bar_speed` is auto-corrected to default (`600`) during load while preserving successful parse.

### TDD Coverage
- Added 9 config-focused tests in `src/config.rs` covering:
  - full valid TOML
  - missing-field defaults
  - invalid background color error
  - empty file defaulting
  - multiple `[[key]]` entries
  - negative `barSpeed` fallback to default
  - `validate_config` warning behavior
  - partial key defaults
  - invalid key color error
