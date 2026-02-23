# KeyOverlay Rust Rewrite

## TL;DR

> **Quick Summary**: Rewrite the C# SFML KeyOverlay app in Rust for cross-platform key press visualization overlays,
> eliminating C# runtime dependency. Feature parity with the original plus CLI args and logging.
>
> **Deliverables**:
> - Cross-platform key overlay binary (Windows, Linux X11+Wayland, macOS)
> - TOML config file with hot-reload support
> - Bundled font for zero-dependency deployment
> - Default config auto-generation on first run
> - Unit tests for all logical modules (TDD approach)
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 5 waves
> **Critical Path**: Rendering Spike → Types+Config → Input Backend → Bar Engine → Integration → Overlay App

---

## Context

### Original Request
Rewrite https://github.com/Friedchicken-42/KeyOverlay from C# to Rust to eliminate C# runtime
dependency and improve cross-platform support. The original is an osu! streaming overlay that shows
animated key press bars with configurable keys, colors, sizes, fading effects, and press counters.

### Interview Summary
**Key Discussions**:
- Config format: Switched from INI to TOML for serde native support and richer types
- Font strategy: Bundle JetBrains Mono (OFL licensed) for zero-dependency deployment
- Wayland: REQUIRED — hybrid input backend with rdev (Win/Mac/X11) + evdev (Wayland)
- Click-through: Overlay must pass input through to underlying windows
- Error handling: Graceful degradation — use defaults for invalid config values, log warnings
- First run: Auto-create default config.toml if not found
- v1 Scope: Feature parity + CLI args + logging. Extended features (tray, KPS, stats, profiles) deferred to v2
- All platforms have equal priority
- TDD for all logical modules (config, input mapping, bar physics, key state)

**Research Findings**:
- egui + egui_overlay: Purpose-built overlay windows, but Wayland support is Xwayland-only. Needs validation spike.
- rdev: RustDesk fork (`rustdesk-org/rdev`) is actively maintained. Original Narsil/rdev abandoned.
- evdev: Raw Linux input, works on Wayland but needs input group permissions.
- toml + serde: Native Rust config parsing, well-supported.
- notify v9 + debouncer: Industry standard file watching.

### Metis Review
**Identified Gaps** (addressed):
- egui_overlay Wayland risk: Added rendering validation spike as Task 1 before any other work
- rdev abandoned: Switched to RustDesk fork (`rustdesk-org/rdev`)
- Scope creep: Locked v1 to parity + CLI + logging; extended features explicitly deferred to v2
- Thread safety: Input listener communicates via std::sync::mpsc channels to render thread
- Font licensing: JetBrains Mono confirmed OFL-licensed (bundling permitted)
- Config migration: Added documentation task for users migrating from original INI format

---

## Work Objectives

### Core Objective
Build a cross-platform transparent overlay application in Rust that displays animated key press
visualizations — colored bars that scroll upward when keys are pressed, with press counters,
fading effects, and hot-reloadable TOML configuration.

### Concrete Deliverables
- `key-overlay-rs` binary (single executable, no runtime deps)
- `config.toml` default configuration file
- Bundled `JetBrainsMono-Bold.ttf` font asset
- Cross-platform builds: Windows (.exe), Linux (binary), macOS (binary)
- Unit tests for: config parsing, key mapping, bar state machine, color parsing, window sizing

### Definition of Done
- [ ] `cargo build --release` succeeds on all platforms
- [ ] `cargo test` passes all unit tests with 0 failures
- [ ] `cargo clippy --all-targets -- -D warnings` has 0 warnings
- [ ] App launches transparent overlay window on Windows
- [ ] App captures global key presses without focus
- [ ] Bars animate upward when keys pressed, stretch when held
- [ ] Config hot-reload works (change config.toml → overlay updates)
- [ ] Default config.toml auto-created if missing

### Must Have
- Global keyboard AND mouse button capture (system-wide, not just app-focused)
- Transparent, borderless, always-on-top, click-through overlay window
- Configurable keys with per-key colors, display names, and width multipliers
- Animated bars that scroll upward on press and stretch on hold
- Delta-time based animation (not frame-dependent)
- Fading gradient effect at top of window
- Per-key press counter display
- TOML config with hot-reload via file watcher
- Graceful error handling (bad config → defaults + warnings, not crash)
- CLI argument for custom config path
- File logging for errors/warnings

### Must NOT Have (Guardrails)
- NO system tray icon (v2)
- NO KPS (keys per second) counter (v2)
- NO session statistics tracking (v2)
- NO profile switching (v2)
- NO GUI config editor (file-editing only)
- NO recording/replay features
- NO network streaming/remote overlay
- NO `unwrap()` or `expect()` in production code paths (use `?` and proper error types)
- NO `as any` or `unsafe` without documented justification
- NO frame-rate-dependent physics (must use delta-time)
- NO hardcoded key mappings (all from config)
- NO blocking the render thread with I/O operations

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — ALL verification is agent-executed. No exceptions.
> Acceptance criteria requiring "user manually tests/confirms" are FORBIDDEN.

### Test Decision
- **Infrastructure exists**: NO (fresh Cargo project)
- **Automated tests**: TDD for logical modules
- **Framework**: Rust built-in `#[cfg(test)]` + `cargo test`
- **If TDD**: Each logical task follows RED (failing test) → GREEN (minimal impl) → REFACTOR

### QA Policy
Every task MUST include agent-executed QA scenarios.
Evidence saved to `.sisyphus/evidence/task-{N}-{scenario-slug}.{ext}`.

- **Rendering/UI**: Use Playwright or screenshot tool to verify overlay window appearance
- **CLI**: Use Bash to run binary with args, validate stdout/stderr output
- **Logic modules**: Use `cargo test` with specific test names
- **Config**: Use `cargo test` for parsing + Bash for file I/O integration

---

## Execution Strategy

### Parallel Execution Waves

> Maximize throughput by grouping independent tasks into parallel waves.
> Each wave completes before the next begins.

```
Wave 0 (Validation — MUST pass before proceeding):
└── Task 1: Rendering spike — validate egui_overlay works [deep]

Wave 1 (Foundation — types, config, assets):
├── Task 2: Project structure + Cargo.toml dependencies [quick]
├── Task 3: Core types + error types (TDD) [deep]
├── Task 4: Color parsing module (TDD) [quick]
├── Task 5: TOML config module (TDD) [deep]
├── Task 6: Key mapping module (TDD) [deep]
└── Task 7: Bundle font asset + font loading [quick]

Wave 2 (Core modules — input, rendering, bars):
├── Task 8: Input backend trait + rdev implementation (TDD) [deep]
├── Task 9: Bar state machine / physics engine (TDD) [deep]
├── Task 10: Window sizing calculator (TDD) [quick]
├── Task 11: Fading effect generator (TDD) [quick]
└── Task 12: Config file watcher + hot-reload [unspecified-high]

Wave 3 (Integration — connect everything):
├── Task 13: Overlay renderer (egui painting) [deep]
├── Task 14: App orchestrator (main loop, threading) [deep]
├── Task 15: CLI argument parsing [quick]
├── Task 16: File logging setup [quick]
└── Task 17: Default config generation + first-run [quick]

Wave 4 (Polish + final verification):
├── Task 18: Integration testing (full pipeline) [deep]
├── Task 19: Cross-platform build verification [unspecified-high]
└── Task 20: README + config documentation [writing]

Wave FINAL (After ALL tasks — independent review, 4 parallel):
├── Task F1: Plan compliance audit (oracle)
├── Task F2: Code quality review (unspecified-high)
├── Task F3: Real manual QA (unspecified-high)
└── Task F4: Scope fidelity check (deep)

Critical Path: T1 → T2 → T3+T5+T6 → T8+T9 → T13+T14 → T18 → FINAL
Parallel Speedup: ~60% faster than sequential
Max Concurrent: 5 (Waves 1 & 2)
```

### Dependency Matrix

| Task | Depends On | Blocks |
|------|-----------|--------|
| T1 | — | T2-T20 (go/no-go gate) |
| T2 | T1 | T3-T17 |
| T3 | T2 | T5, T6, T8, T9, T10, T13, T14 |
| T4 | T2 | T5, T9, T11, T13 |
| T5 | T2, T3, T4 | T8, T12, T13, T14, T17 |
| T6 | T2, T3 | T8, T9, T13 |
| T7 | T2 | T13 |
| T8 | T3, T5, T6 | T14 |
| T9 | T3, T4, T6 | T13, T14 |
| T10 | T3, T5 | T13 |
| T11 | T3, T4 | T13 |
| T12 | T5 | T14 |
| T13 | T3-T7, T9, T10, T11 | T14, T18 |
| T14 | T5, T8, T9, T12, T13 | T18 |
| T15 | T2 | T14 |
| T16 | T2 | T14 |
| T17 | T5 | T14 |
| T18 | T14 | FINAL |
| T19 | T14 | FINAL |
| T20 | T5, T14 | FINAL |

### Agent Dispatch Summary

- **Wave 0**: 1 task — T1 → `deep`
- **Wave 1**: 6 tasks — T2 → `quick`, T3 → `deep`, T4 → `quick`, T5 → `deep`, T6 → `deep`, T7 → `quick`
- **Wave 2**: 5 tasks — T8 → `deep`, T9 → `deep`, T10 → `quick`, T11 → `quick`, T12 → `unspecified-high`
- **Wave 3**: 5 tasks — T13 → `deep`, T14 → `deep`, T15 → `quick`, T16 → `quick`, T17 → `quick`
- **Wave 4**: 3 tasks — T18 → `deep`, T19 → `unspecified-high`, T20 → `writing`
- **FINAL**: 4 tasks — F1 → `oracle`, F2 → `unspecified-high`, F3 → `unspecified-high`, F4 → `deep`

---

## TODOs

 [x] 1. Rendering Spike — Validate egui_overlay ✅ GO

  **What to do**:
  - Create a minimal egui_overlay application that:
    1. Opens a transparent, always-on-top, click-through window
    2. Draws a filled rectangle with stroke outline using egui Painter API
    3. Renders text at a specific position
    4. Runs at 60fps with smooth frame timing
    5. Accepts keyboard/mouse events OR demonstrates input passthrough
  - Test on Windows (primary dev platform) — document behavior on each platform
  - If egui_overlay FAILS transparency/click-through: document the failure and evaluate
    alternatives (winit+softbuffer, SDL2, raw platform APIs). The plan will need revision.
  - This is a GO/NO-GO gate: if the spike fails, the entire rendering approach changes

  **Must NOT do**:
  - Do NOT build any app logic — this is a throwaway spike
  - Do NOT spend more than a few hours — fast validation, not polish
  - Do NOT attempt Wayland testing in the spike (egui_overlay uses Xwayland)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: Requires exploring unfamiliar crate APIs, debugging platform-specific behavior
  - **Skills**: [`rust-pro`]
    - `rust-pro`: Rust idioms and crate usage patterns
  - **Skills Evaluated but Omitted**:
    - `visual-engineering`: Not needed — no design work, just API validation

  **Parallelization**:
  - **Can Run In Parallel**: NO
  - **Parallel Group**: Wave 0 (solo, blocking gate)
  - **Blocks**: ALL subsequent tasks (T2-T20)
  - **Blocked By**: None (can start immediately)

  **References**:

  **Pattern References**:
  - Original C# AppWindow.cs rendering approach: creates RenderWindow with VideoMode, draws RectangleShape + Text

  **External References**:
  - `egui_overlay` crate: https://crates.io/crates/egui_overlay — OverlayApp trait, transparent windows
  - `egui` painter API: `ui.painter().rect_stroke()`, `ui.painter().rect_filled()`, `ui.painter().text()`
  - egui_overlay examples: check crate's examples/ directory on GitHub for setup patterns

  **WHY Each Reference Matters**:
  - The original C# uses SFML RenderWindow — we need to verify egui can achieve equivalent results
  - The painter API is how we'll draw ALL visual elements — must work with alpha/transparency

  **Acceptance Criteria**:
  - [ ] `cargo build` compiles without errors
  - [ ] `cargo run` opens a window
  - [ ] Window is transparent (desktop visible through empty areas)
  - [ ] Window is always-on-top (stays above other windows)
  - [ ] A colored filled rectangle is visible
  - [ ] Text is rendered at a specified position
  - [ ] Application runs without crashing for 10+ seconds

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Overlay window renders with transparency
    Tool: Bash
    Preconditions: egui_overlay spike code compiled
    Steps:
      1. Run `cargo run` (timeout 15s, then kill)
      2. While running, take screenshot via `screenshot` skill
      3. Verify process started without panic (check stderr)
    Expected Result: Process runs for 15s without crash, exit code 0 or killed
    Failure Indicators: Panic message in stderr, immediate crash, "cannot create window" error
    Evidence: .sisyphus/evidence/task-1-overlay-spike.txt

  Scenario: Compilation succeeds with all dependencies
    Tool: Bash
    Preconditions: Cargo.toml has egui_overlay dependency
    Steps:
      1. Run `cargo build 2>&1`
      2. Check exit code is 0
      3. Verify no warnings related to egui_overlay
    Expected Result: Build succeeds, binary exists in target/debug/
    Failure Indicators: Compilation errors, missing system dependencies
    Evidence: .sisyphus/evidence/task-1-build-check.txt
  ```

  **Commit**: YES
  - Message: `spike(overlay): validate egui_overlay transparency and drawing`
  - Files: `src/main.rs`, `Cargo.toml`
  - Pre-commit: `cargo build`

- [ ] 2. Project Structure + Cargo.toml Dependencies

  **What to do**:
  - Set up the full project structure (modules, directories) based on the architecture:
    ```
    Cargo.toml
    assets/
      JetBrainsMono-Bold.ttf  (placeholder — actual font in T7)
    src/
      main.rs       # Entry point — arg parsing + app launch
      lib.rs        # Re-exports public modules
      types.rs      # Core domain types (Color, KeyConfig, etc.)
      color.rs      # Color parsing and conversion
      config.rs     # TOML config loading and validation
      key_map.rs    # Key string → rdev Key/Button mapping
      bars.rs       # Bar state machine and physics
      layout.rs     # Window sizing calculations
      fading.rs     # Fading gradient generation
      renderer.rs   # egui overlay rendering
      app.rs        # Main app orchestrator
      cli.rs        # CLI argument parsing (clap)
      logging.rs    # File logging setup (tracing)
      watcher.rs    # Config file watcher
      font.rs       # Font loading utility
      input/
        mod.rs            # InputBackend trait definition
        rdev_backend.rs   # rdev-based implementation (Win/Mac/X11)
    ```
  - Add ALL dependencies to Cargo.toml:
    - `egui` + `egui_overlay` (rendering)
    - `rdev` (from rustdesk-org fork via git) (input capture)
    - `serde` + `serde_derive` + `toml` (config parsing)
    - `notify` + `notify-debouncer-full` (file watching)
    - `anyhow` + `thiserror` (error handling)
    - `clap` with derive feature (CLI args)
    - `tracing` + `tracing-subscriber` + `tracing-appender` (logging)
    - `chrono` or `std::time` (delta time)
  - Each module file should have a module doc comment and stub public API
  - `lib.rs` should declare all modules with `pub mod`
  - Replace spike code in main.rs with placeholder that prints version

  **Must NOT do**:
  - Do NOT implement any logic — just module structure and stubs
  - Do NOT add `unsafe` code
  - Do NOT add platform-specific `#[cfg]` blocks yet (those come in T8)

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Mechanical scaffolding task, no complex logic
  - **Skills**: [`rust-pro`]
    - `rust-pro`: Cargo.toml configuration, module structure patterns
  - **Skills Evaluated but Omitted**:
    - `architecture-patterns`: Overkill for file scaffolding

  **Parallelization**:
  - **Can Run In Parallel**: NO (foundation for all other tasks)
  - **Parallel Group**: Wave 1 (first task to complete)
  - **Blocks**: T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17
  - **Blocked By**: T1 (spike must pass first)

  **References**:

  **Pattern References**:
  - AGENTS.md module structure section — follow the prescribed directory layout
  - AGENTS.md import ordering — std, external, crate, super/self

  **External References**:
  - `clap` derive: https://docs.rs/clap/latest/clap/_derive/index.html
  - `tracing` setup: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/
  - `egui_overlay` on crates.io for exact version
  - `rustdesk-org/rdev` GitHub repo for git dependency syntax

  **WHY Each Reference Matters**:
  - AGENTS.md defines the coding style contract — agents must follow it
  - The clap derive pattern determines CLI module structure
  - rdev needs a git dependency (fork), not a crates.io version

  **Acceptance Criteria**:
  - [ ] `cargo check` succeeds with 0 errors
  - [ ] `cargo clippy --all-targets -- -D warnings` has 0 warnings
  - [ ] All module files exist at expected paths
  - [ ] `lib.rs` declares all modules
  - [ ] `Cargo.toml` lists all required dependencies

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Project compiles with all dependencies resolved
    Tool: Bash
    Preconditions: None
    Steps:
      1. Run `cargo check 2>&1`
      2. Assert exit code is 0
      3. Run `cargo clippy --all-targets -- -D warnings 2>&1`
      4. Assert exit code is 0
    Expected Result: Both commands succeed with no errors or warnings
    Failure Indicators: Dependency resolution errors, missing crate features, module declaration errors
    Evidence: .sisyphus/evidence/task-2-cargo-check.txt

  Scenario: All module files exist
    Tool: Bash
    Preconditions: Scaffolding complete
    Steps:
      1. Run `ls src/*.rs src/input/*.rs`
      2. Verify each expected file is listed
    Expected Result: All files from the structure above exist
    Failure Indicators: Missing files, wrong directory structure
    Evidence: .sisyphus/evidence/task-2-file-structure.txt
  ```

  **Commit**: YES
  - Message: `chore(init): project structure and dependencies`
  - Files: `Cargo.toml`, `src/**/*.rs`
  - Pre-commit: `cargo check`

- [ ] 3. Core Types + Error Types (TDD)

  **What to do**:
  - **TDD: Write tests FIRST, then implement.**
  - Define core domain types in `src/types.rs`:
    ```rust
    /// RGBA color with f32 components (0.0-1.0) for egui compatibility
    pub struct Color { pub r: f32, pub g: f32, pub b: f32, pub a: f32 }
    
    /// Configuration for a single monitored key
    pub struct KeyConfig {
      pub key_name: String,      // rdev key identifier string
      pub display_name: String,  // text shown on overlay
      pub color: Color,          // bar/outline color
      pub size: f32,             // width multiplier (1.0 = normal)
    }
    
    /// Full application configuration
    pub struct AppConfig {
      pub height: f32,
      pub key_size: f32,
      pub bar_speed: f32,
      pub background_color: Color,
      pub margin: f32,
      pub outline_thickness: f32,
      pub fading: bool,
      pub counter: bool,
      pub fps: u32,
      pub keys: Vec<KeyConfig>,
    }
    
    /// Represents an input event from the backend
    pub enum InputEvent {
      KeyPress(String),    // key identifier
      KeyRelease(String),
      MousePress(String),  // mouse button identifier
      MouseRelease(String),
    }
    ```
  - Implement `Default` for `AppConfig` with sensible defaults matching the original:
    height=700, key_size=70, bar_speed=600, background_color=black(0,0,0,255),
    margin=25, outline_thickness=5, fading=true, counter=true, fps=60,
    keys=[Z, X] (minimal default set)
  - Define error types in `src/types.rs` (or separate error module) using `thiserror`:
    ```rust
    #[derive(thiserror::Error, Debug)]
    pub enum AppError {
      #[error("Config error: {0}")] Config(String),
      #[error("Input backend error: {0}")] Input(String),
      #[error("Render error: {0}")] Render(String),
      #[error("IO error: {0}")] Io(#[from] std::io::Error),
    }
    ```
  - Implement `Color` methods: `new(r,g,b,a)`, `pressed()` (alpha/1.618 golden ratio dimming),
    `to_egui()` → `egui::Color32`, `from_rgba_u8(r,g,b,a)`
  - Write tests for:
    - Default AppConfig values match expected
    - Color conversion (u8 ↔ f32)
    - Color::pressed() applies golden ratio alpha dimming
    - KeyConfig creation

  **Must NOT do**:
  - Do NOT implement config file parsing (that's T5)
  - Do NOT import or depend on egui types in tests (keep types pure)

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: TDD workflow with careful type design requires thoughtful implementation
  - **Skills**: [`rust-pro`, `tdd-workflow`]
    - `rust-pro`: Rust type system patterns, derive macros
    - `tdd-workflow`: RED-GREEN-REFACTOR discipline
  - **Skills Evaluated but Omitted**:
    - `error-handling-patterns`: thiserror usage is straightforward

  **Parallelization**:
  - **Can Run In Parallel**: YES (with T4, T7 after T2 completes)
  - **Parallel Group**: Wave 1 (with T4, T5, T6, T7)
  - **Blocks**: T5, T6, T8, T9, T10, T13, T14
  - **Blocked By**: T2

  **References**:

  **Pattern References**:
  - Original Key.cs: `_color`, `_colorPressed` (alpha / 1.618), `_size` fields
  - Original Config.cs: section structure maps to AppConfig fields
  - Original config.ini: default values (height=700, keySize=70, barSpeed=600, etc.)

  **External References**:
  - `thiserror` derive macro: https://docs.rs/thiserror/latest/thiserror/
  - `egui::Color32`: for target conversion type reference

  **WHY Each Reference Matters**:
  - The original Key.cs shows the golden ratio (1.618) color dimming — must replicate exactly
  - Default values from config.ini are the expected defaults for AppConfig::default()

  **Acceptance Criteria**:
  - [ ] `cargo test types` passes all tests
  - [ ] Color::pressed() returns alpha divided by 1.618
  - [ ] AppConfig::default() matches original config.ini defaults
  - [ ] All types derive Debug, Clone
  - [ ] Error types use thiserror derive

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: All type tests pass
    Tool: Bash
    Preconditions: types.rs implemented
    Steps:
      1. Run `cargo test types -- --nocapture 2>&1`
      2. Assert exit code is 0
      3. Verify test count >= 4 (default config, color conversion, pressed color, key config)
    Expected Result: All tests pass, output shows test names and results
    Failure Indicators: Any test failure, compilation error
    Evidence: .sisyphus/evidence/task-3-types-tests.txt

  Scenario: Color pressed applies golden ratio dimming
    Tool: Bash
    Preconditions: Color::pressed() implemented
    Steps:
      1. Run `cargo test color_pressed -- --nocapture 2>&1`
      2. Verify output shows alpha value is original_alpha / 1.618
    Expected Result: Test passes, alpha dimming correct to 3 decimal places
    Failure Indicators: Wrong alpha value, integer truncation
    Evidence: .sisyphus/evidence/task-3-color-pressed.txt
  ```

  **Commit**: YES (groups with T4)
  - Message: `feat(types): core types, error types, and color parsing with TDD`
  - Files: `src/types.rs`
  - Pre-commit: `cargo test types`

- [ ] 4. Color Parsing Module (TDD)

  **What to do**:
  - **TDD: Write tests FIRST, then implement.**
  - Create `src/color.rs` with functions for parsing color strings:
    - `parse_color(s: &str) -> Result<Color>`: Parse "R,G,B,A" string (u8 values 0-255) to Color
    - `parse_color_or_default(s: &str, default: Color) -> Color`: Graceful fallback
    - Handle edge cases: extra whitespace, missing alpha (default 255), invalid numbers
  - Write tests for:
    - Valid RGBA: "255,0,128,200" → Color
    - Missing alpha: "255,0,128" → Color with alpha 1.0 (255)
    - Invalid format: "not,a,color" → Error
    - Empty string → Error
    - Whitespace: " 255 , 0 , 128 , 200 " → Color (trimmed)
    - Out of range: "300,0,0,0" → Error or clamped (decide: clamp to 255)

  **Must NOT do**:
  - Do NOT depend on config module (pure parsing functions)
  - Do NOT handle anything beyond single color string parsing

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: Simple parsing logic with clear test cases
  - **Skills**: [`tdd-workflow`]
    - `tdd-workflow`: RED-GREEN-REFACTOR for parsing edge cases
  - **Skills Evaluated but Omitted**:
    - `rust-pro`: Not needed for simple string parsing

  **Parallelization**:
  - **Can Run In Parallel**: YES (with T3, T5, T6, T7)
  - **Parallel Group**: Wave 1
  - **Blocks**: T5, T9, T11, T13
  - **Blocked By**: T2

  **References**:

  **Pattern References**:
  - Original CreateItems.cs `CreateColor()`: splits on ',' and creates `new Color(byte, byte, byte, byte)`
  - Original config.ini color format: `backgroundColor=0,0,0,255`, `key2=0,255,255,255`

  **WHY Each Reference Matters**:
  - The exact color string format from the original must be supported (R,G,B,A with u8 values)
  - CreateColor shows the parsing logic to replicate

  **Acceptance Criteria**:
  - [ ] `cargo test color` passes all tests
  - [ ] Valid "R,G,B,A" strings parse correctly
  - [ ] Missing alpha defaults to 255 (fully opaque)
  - [ ] Invalid strings return Error (not panic)
  - [ ] Whitespace is handled gracefully

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Color parsing tests pass
    Tool: Bash
    Preconditions: color.rs implemented
    Steps:
      1. Run `cargo test color -- --nocapture 2>&1`
      2. Assert exit code is 0
      3. Verify test count >= 5
    Expected Result: All parsing tests pass
    Failure Indicators: Panic on invalid input, wrong color values
    Evidence: .sisyphus/evidence/task-4-color-tests.txt

  Scenario: Graceful error on invalid color string
    Tool: Bash
    Preconditions: parse_color implemented
    Steps:
      1. Run `cargo test color_invalid -- --nocapture 2>&1`
      2. Verify test passes (returns Err, not panic)
    Expected Result: Error returned with descriptive message, no panic
    Failure Indicators: Thread panic, unwrap failure
    Evidence: .sisyphus/evidence/task-4-color-error.txt
  ```

  **Commit**: YES (groups with T3)
  - Message: `feat(types): core types, error types, and color parsing with TDD`
  - Files: `src/color.rs`
  - Pre-commit: `cargo test color`

- [ ] 5. TOML Config Module (TDD)

  **What to do**:
  - **TDD: Write tests FIRST, then implement.**
  - Create `src/config.rs` that loads and validates TOML configuration:
    - Define a `RawConfig` struct with serde Deserialize for TOML parsing (all fields Optional for graceful defaults)
    - Define `load_config(path: &Path) -> Result<AppConfig>` that:
      1. Reads file to string
      2. Deserializes to RawConfig via toml::from_str
      3. Validates and converts to AppConfig, filling defaults for missing fields
      4. Logs warnings for invalid values (e.g., negative bar_speed) and substitutes defaults
    - Parse TOML key table format:
      ```toml
      [general]
      height = 700
      keySize = 70
      barSpeed = 600
      backgroundColor = "0,0,0,255"
      margin = 25
      outlineThickness = 5
      fading = true
      counter = true
      fps = 60

      [[key]]
      name = "Z"
      color = "255,0,0,255"
      size = 1.0

      [[key]]
      name = "X"
      color = "0,255,255,255"
      size = 1.0
      ```
    - Use `color.rs::parse_color()` (from T4) for color string fields
    - Implement `validate_config(config: &AppConfig) -> Vec<String>` that returns warnings
    - Implement `AppConfig::default()` matching original defaults exactly
  - Write tests for: full valid TOML, missing optional fields, invalid color strings, empty file,
    missing [general] section, single/multiple keys, missing key fields, negative values

  **Must NOT do**:
  - Do NOT implement file watching (T12), CLI config path (T15), or read from disk in tests

  **Recommended Agent Profile**:
  - **Category**: `deep` — TDD with complex deserialization logic
  - **Skills**: [`rust-pro`, `tdd-workflow`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with T6, T7)
  - **Parallel Group**: Wave 1
  - **Blocks**: T8, T12, T13, T14, T17
  - **Blocked By**: T2, T3, T4

  **References**:
  - Original Config.cs: `ReadConfig()` method with fallback defaults
  - Original config.ini: exact field names and default values
  - `src/types.rs` (T3): AppConfig, KeyConfig structs
  - `src/color.rs` (T4): parse_color() function
  - `toml` crate: https://docs.rs/toml/latest/toml/
  - `serde` derive: https://serde.rs/derive.html — `#[serde(default)]`, `#[serde(rename)]`

  **Acceptance Criteria**:
  - [ ] `cargo test config -- --nocapture` passes all tests (>= 8 tests)
  - [ ] Valid TOML with all fields parses to correct AppConfig
  - [ ] Missing fields produce defaults (not panics)
  - [ ] Invalid values produce warnings (not crashes)
  - [ ] Empty TOML -> full default config
  - [ ] Multiple [[key]] entries parse into Vec<KeyConfig>

  **QA Scenarios (MANDATORY):**

  ```
  Scenario: Full config parsing tests pass
    Tool: Bash
    Steps:
      1. Run `cargo test config -- --nocapture 2>&1`
      2. Assert exit code is 0, test count >= 8
    Evidence: .sisyphus/evidence/task-5-config-tests.txt

  Scenario: Empty TOML produces valid default config
    Tool: Bash
    Steps:
      1. Run `cargo test config_empty -- --nocapture 2>&1`
      2. Verify default height=700, key_size=70, bar_speed=600
    Evidence: .sisyphus/evidence/task-5-config-empty.txt
  ```

  **Commit**: YES (groups with T6)
  - Message: `feat(config): TOML config parser and key mapping with TDD`
  - Files: `src/config.rs`
  - Pre-commit: `cargo test config`

- [ ] 6. Key Mapping Module (TDD)

  **What to do**:
  - Create `src/input/key_mapping.rs` with `KeyId` enum covering keyboard keys + mouse buttons
  - Implement `FromStr` for `KeyId` to parse config strings ("A", "Z", "Mouse1", "Space", etc.)
  - Map `KeyId ↔ rdev::Key` and `KeyId ↔ evdev::Key` with bidirectional conversion traits
  - Support the original KeyOverlay naming: e.g. `"LControl"`, `"RShift"`, `"Mouse1"`–`"Mouse5"`
  - Include `Display` impl for user-facing labels (what appears on the overlay)
  - TDD: Write tests first for parsing, round-trip conversion, unknown key handling

  **Must NOT do**: Don't pull in platform crates at compile time for all platforms — use `#[cfg]` gating. Don't support joystick/gamepad input.

  **Recommended Agent Profile**:
  - **Category**: `deep` — TDD + cross-platform cfg gating logic
  - **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T3, T4, T5, T7)
  - **Blocks**: T8, T9
  - **Blocked By**: T2

  **References**:
  - Original C# `Key.cs` — key names from config (`_keyString` field, keyboard enum values)
  - `rdev::Key` enum — https://docs.rs/rdev/latest/rdev/enum.Key.html
  - `evdev::Key` constants — https://docs.rs/evdev/latest/evdev/struct.Key.html
  - T3 `src/types.rs` — `KeyId` type should be defined or re-exported from here

  **Acceptance Criteria**:
  - [ ] `cargo test key_mapping` → all pass
  - [ ] All original KeyOverlay key names parse correctly (A-Z, 0-9, F1-F12, Mouse1-5, modifiers)
  - [ ] Unknown key string returns descriptive error, not panic
  - [ ] `#[cfg(target_os)]` gating compiles on all platforms

  **QA Scenarios**:
  ```
  Scenario: Parse all supported key names round-trip
    Tool: Bash
    Steps:
      1. cargo test key_mapping -- --nocapture
      2. Verify output includes tests for: alphabetic, numeric, function, modifier, mouse keys
      3. Verify all tests PASS
    Expected: 0 failures, coverage of ≥20 distinct key names
    Evidence: .sisyphus/evidence/task-6-key-mapping-roundtrip.txt

  Scenario: Invalid key name produces error
    Tool: Bash
    Steps:
      1. cargo test key_mapping_invalid -- --nocapture
      2. Verify test asserts Err variant with descriptive message
    Expected: Error contains the invalid key name for debugging
    Evidence: .sisyphus/evidence/task-6-key-mapping-invalid.txt
  ```

  **Commit**: YES (groups with T5)
  - Message: `feat(config): TOML config parser and key mapping with TDD`
  - Files: `src/input/key_mapping.rs`, `src/input/mod.rs`
  - Pre-commit: `cargo test key_mapping`

- [ ] 7. Bundle Font Asset + Font Loading

  **What to do**:
  - Add `assets/` directory with a bundled font (Roboto Mono or similar OFL-licensed monospace font)
  - Create `src/font.rs` — embed font bytes via `include_bytes!` macro at compile time
  - Expose `pub fn load_font() -> &'static [u8]` returning the embedded font data
  - Add `LICENSE-FONT` file in `assets/` with the font's OFL license text
  - Keep it simple: one font, one weight, no runtime font discovery

  **Must NOT do**: Don't use system font lookup. Don't support user-supplied fonts (v2). Don't embed multiple weights.

  **Recommended Agent Profile**:
  - **Category**: `quick` — simple asset embedding, no complex logic
  - **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with T3, T4, T5, T6)
  - **Blocks**: T13
  - **Blocked By**: T2

  **References**:
  - `include_bytes!` macro — https://doc.rust-lang.org/std/macro.include_bytes.html
  - Google Fonts Roboto Mono — OFL licensed monospace font
  - Original C# uses `new Font("Arial", ...)` — we replace with bundled monospace

  **Acceptance Criteria**:
  - [ ] `assets/` directory contains `.ttf` file + `LICENSE-FONT`
  - [ ] `cargo build` succeeds with embedded font
  - [ ] `load_font()` returns non-empty byte slice
  - [ ] Binary size increase is < 500KB from font embedding

  **QA Scenarios**:
  ```
  Scenario: Font embeds and loads correctly
    Tool: Bash
    Steps:
      1. cargo build 2>&1
      2. Verify build succeeds with no errors
      3. cargo test font -- --nocapture
      4. Verify test confirms byte slice length > 0
    Expected: Build success, font bytes accessible at runtime
    Evidence: .sisyphus/evidence/task-7-font-embed.txt
  ```

  **Commit**: YES
  - Message: `feat(assets): bundle Roboto Mono font with OFL license`
  - Files: `assets/RobotoMono-Regular.ttf`, `assets/LICENSE-FONT`, `src/font.rs`
  - Pre-commit: `cargo build`

- [ ] 8. Input Backend Trait + rdev Implementation (TDD)

  **What to do**:
  - Create `src/input/backend.rs` — define `InputBackend` trait:
    ```rust
    pub trait InputBackend: Send + 'static {
        fn start(&mut self, tx: crossbeam_channel::Sender<InputEvent>) -> Result<()>;
        fn stop(&mut self) -> Result<()>;
    }
    ```
  - Define `InputEvent` enum: `KeyPress(KeyId)`, `KeyRelease(KeyId)` in `src/input/event.rs`
  - Implement `RdevBackend` in `src/input/rdev_backend.rs`:
    - Spawn `rdev::listen` on dedicated thread (it blocks)
    - Convert `rdev::Event` → `InputEvent` using T6 key mappings
    - Send events through `crossbeam_channel::Sender`
    - Handle `rdev::listen` errors gracefully (log + retry or fatal)
  - Implement `EvdevBackend` in `src/input/evdev_backend.rs` (cfg-gated `target_os = "linux"`):
    - Open `/dev/input/event*` devices with read permission
    - Filter for keyboard/mouse devices
    - Poll with `epoll` or async, convert to `InputEvent`
  - Factory function: `pub fn create_backend() -> Box<dyn InputBackend>` — picks backend by platform
  - TDD: Test event conversion, mock backend for downstream consumers

  **Must NOT do**: Don't request elevated privileges silently. Don't grab exclusive device access. Don't handle hotplug (v2).

  **Recommended Agent Profile**:
  - **Category**: `deep` — cross-platform input, threading, TDD
  - **Skills**: [`rust-pro`, `rust-async-patterns`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T9, T10, T11, T12)
  - **Blocks**: T14
  - **Blocked By**: T3, T6

  **References**:
  - `rdev` crate (rustdesk-org fork) — `rdev::listen()`, `rdev::Event`, `rdev::EventType`
  - `evdev` crate — `Device::open()`, `InputEvent`, `InputEventKind`
  - `crossbeam-channel` — `unbounded()` sender/receiver for thread-safe event passing
  - T6 `src/input/key_mapping.rs` — `KeyId` conversion from platform key types
  - Original C# `Key.cs` lines 52-97 — `KeyPress`/`KeyRelease` SFML event handlers

  **Acceptance Criteria**:
  - [ ] `cargo test input` → all pass
  - [ ] `InputBackend` trait is object-safe (`Box<dyn InputBackend>` compiles)
  - [ ] `RdevBackend` compiles on Windows/macOS/Linux-X11
  - [ ] `EvdevBackend` compiles only on Linux (`#[cfg(target_os = "linux")]`)
  - [ ] Mock backend exists for testing downstream consumers without real input devices
  - [ ] `cargo clippy -- -D warnings` passes for input module

  **QA Scenarios**:
  ```
  Scenario: Mock backend sends events through channel
    Tool: Bash
    Steps:
      1. cargo test input::tests::mock_backend -- --nocapture
      2. Verify test creates MockBackend, sends KeyPress+KeyRelease, receives on channel
      3. Verify event KeyId matches what was sent
    Expected: Events flow correctly through the trait interface
    Evidence: .sisyphus/evidence/task-8-mock-backend.txt

  Scenario: rdev event conversion handles all key types
    Tool: Bash
    Steps:
      1. cargo test input::tests::rdev_conversion -- --nocapture
      2. Verify conversion of rdev::Key::KeyA → KeyId::A
      3. Verify unknown rdev keys produce KeyId::Unknown with raw code
    Expected: All mapped keys convert correctly, unknowns don't panic
    Evidence: .sisyphus/evidence/task-8-rdev-conversion.txt
  ```

  **Commit**: YES
  - Message: `feat(input): input backend trait with rdev + evdev implementations (TDD)`
  - Files: `src/input/backend.rs`, `src/input/event.rs`, `src/input/rdev_backend.rs`, `src/input/evdev_backend.rs`, `src/input/mod.rs`
  - Pre-commit: `cargo test input`

- [ ] 9. Bar State Machine / Physics Engine (TDD)

  **What to do**:
  - **TDD: Write tests FIRST, then implement.**
  - Create `src/bars.rs` with the bar animation engine:
    - `Bar` struct: `{ y_position: f32, height: f32, color: Color, pressed_color: Color }`
    - `BarColumn` struct: manages bars for one key — holds `Vec<Bar>`, `is_held: bool`, `hold_count: u32`, `press_count: u64`
    - On `KeyPress`: if `hold_count == 0`, push new `Bar` at bottom (y=0, height=1px). Set `is_held = true`, `hold_count = 1`, increment `press_count`
    - On sustained hold (`hold_count > 0` and still held): stretch last bar's height (grows while held)
    - On `KeyRelease`: set `is_held = false`, reset `hold_count = 0`
    - `update(dt: f32)`: move ALL bars upward by `bar_speed * dt`. If held, also grow last bar by `bar_speed * dt`
    - `remove_offscreen(window_height: f32)`: remove bars where `y_position > window_height + bar_height`
    - Colors: normal bar uses key color, pressed state uses `color.pressed()` (alpha/1.618)
  - `BarManager`: owns `HashMap<KeyId, BarColumn>`, dispatches input events to correct column
  - All physics must use delta-time (f32 seconds), never frame-count
  - TDD: test bar creation, stretching, movement, offscreen removal, press counting

  **Must NOT do**: No rendering code. No egui imports. Pure state + math only. No frame-dependent logic.

  **Recommended Agent Profile**:
  - **Category**: `deep` — complex state machine with TDD
  - **Skills**: [`rust-pro`, `tdd-workflow`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T10, T11, T12)
  - **Blocks**: T13, T14
  - **Blocked By**: T3, T4, T6

  **References**:
  - Original C# `Key.cs` lines 52-120: `KeyPress`/`KeyRelease` handlers, `Hold` counter, bar creation/stretching logic
  - Original C# `Key.cs` `Update()`: `bar.Position.Y -= barSpeed * delta`, removal when `position.Y + size.Y < 0`
  - Original C# `Key.cs` pressed color: `new Color(c.R, c.G, c.B, (byte)(c.A / 1.618f))`
  - T3 `src/types.rs` — `Color`, `KeyConfig`, `InputEvent` types
  - T6 `src/input/key_mapping.rs` — `KeyId` enum for HashMap keys

  **Acceptance Criteria**:
  - [ ] `cargo test bars` → all pass (≥8 tests)
  - [ ] New bar created on KeyPress with correct initial position
  - [ ] Bar stretches while key held (height increases with dt)
  - [ ] Bars move upward by `bar_speed * dt` each update
  - [ ] Offscreen bars removed (y_position beyond window height)
  - [ ] Press counter increments on each KeyPress, not on hold
  - [ ] Delta-time based — passing dt=0.0 causes no movement

  **QA Scenarios**:
  ```
  Scenario: Bar lifecycle — create, move, remove
    Tool: Bash
    Steps:
      1. cargo test bars::tests::bar_lifecycle -- --nocapture
      2. Verify: new bar at y=0, after update(0.016) bar moved up, after many updates bar removed
    Expected: Bar moves by bar_speed*dt per frame, removed when offscreen
    Evidence: .sisyphus/evidence/task-9-bar-lifecycle.txt

  Scenario: Hold stretches bar
    Tool: Bash
    Steps:
      1. cargo test bars::tests::hold_stretch -- --nocapture
      2. Verify: KeyPress creates bar, hold_count increments, bar height grows with dt
      3. Verify: KeyRelease stops stretching
    Expected: Bar height > initial after hold period
    Evidence: .sisyphus/evidence/task-9-hold-stretch.txt
  ```

  **Commit**: YES
  - Message: `feat(bars): bar state machine and physics engine with TDD`
  - Files: `src/bars.rs`
  - Pre-commit: `cargo test bars`

- [ ] 10. Window Sizing Calculator (TDD)

  **What to do**:
  - **TDD: Write tests FIRST, then implement.**
  - Create `src/layout.rs` with window sizing logic from original C# `CreateItems.cs`:
    - `calculate_window_width(config: &AppConfig) -> f32`:
      `margin + sum_over_keys(key_size * key.size + outline_thickness * 2 + margin)`
    - `calculate_key_x_positions(config: &AppConfig) -> Vec<f32>`: returns x-offset for each key column
    - `calculate_column_width(key_size: f32, key_size_multiplier: f32, outline_thickness: f32) -> f32`
  - These are pure functions: config in, numbers out. No side effects.
  - TDD: test with known config values and verify exact pixel positions match original formula

  **Must NOT do**: No window creation. No egui types. Pure math only.

  **Recommended Agent Profile**:
  - **Category**: `quick` — pure math functions with TDD
  - **Skills**: [`tdd-workflow`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T9, T11, T12)
  - **Blocks**: T13
  - **Blocked By**: T3, T5

  **References**:
  - Original C# `CreateItems.cs` line ~50: `width = margin + Σ(keySize * key._size + outlineThickness * 2 + margin)`
  - Original C# `CreateItems.cs`: calculates x-positions for each key column sequentially
  - T3 `src/types.rs` — `AppConfig`, `KeyConfig` types

  **Acceptance Criteria**:
  - [ ] `cargo test layout` → all pass
  - [ ] Window width matches formula: margin + Σ(key_size * size + outline*2 + margin)
  - [ ] 2 keys of size 1.0 with key_size=70, margin=25, outline=5 → width = 25 + (70+10+25)*2 = 235
  - [ ] Key x-positions are sequential, non-overlapping

  **QA Scenarios**:
  ```
  Scenario: Window width calculation matches formula
    Tool: Bash
    Steps:
      1. cargo test layout::tests::window_width -- --nocapture
      2. Verify calculated width for known config matches hand-computed value
    Expected: Exact f32 match for test cases
    Evidence: .sisyphus/evidence/task-10-window-width.txt
  ```

  **Commit**: YES (groups with T11)
  - Message: `feat(layout): window sizing calculator and fading effect`
  - Files: `src/layout.rs`
  - Pre-commit: `cargo test layout`

- [ ] 11. Fading Effect Generator (TDD)

  **What to do**:
  - Create `src/fading.rs` with a `FadingStrip` struct (y_offset, height, alpha) and a `generate_fading_strips(config) -> Vec<FadingStrip>` function
  - Port the original algorithm: 255 strips, each `2 * ratio_y` pixels tall (ratio_y = key_height / 255.0), alpha decreasing from 255 to 0
  - Each strip: `y = -(i as f32) * strip_height`, `alpha = 255 - i as u8`
  - Provide `apply_fading_color(base_color: Color, strip_alpha: u8) -> Color` — multiplies base alpha by strip_alpha/255
  - All pure functions, no side effects — ideal for TDD
  - Unit tests: exact strip count (255), total height = 2 * key_height, first strip alpha=255, last strip alpha=0, fading color blending math

  **Must NOT do**: Render anything; this is pure data generation. No egui dependency.

  **Recommended Agent Profile**:
  - **Category**: `quick` — **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T9, T10, T12)
  - **Blocks**: T13 (renderer needs fading strips)
  - **Blocked By**: T3 (Color type)

  **References**:
  - `src/types.rs` — Color struct with alpha field
  - Original C# `Fading.cs`: 255 strips, `2 * ratioY` height, alpha from 255→0
  - Original formula: `ratioY = keyHeight / 255f`, strip height = `2 * ratioY`

  **Acceptance Criteria**:
  - [ ] `src/fading.rs` exists with `FadingStrip` and `generate_fading_strips()`
  - [ ] `cargo test fading` passes — strip count, heights, alpha range verified
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Fading strip generation correctness
    Tool: Bash
    Steps:
      1. cargo test fading::tests -- --nocapture
      2. Verify 255 strips generated, alpha range 255→0, total height ≈ 2 * key_height
    Expected: All assertions pass
    Evidence: .sisyphus/evidence/task-11-fading-strips.txt
  ```

  **Commit**: YES (groups with T10)
  - Message: `feat(layout): window sizing calculator and fading effect`
  - Files: `src/fading.rs`
  - Pre-commit: `cargo test fading`

- [ ] 12. Config File Watcher with Hot-Reload

  **What to do**:
  - Create `src/watcher.rs` using `notify` (v9) + `notify-debouncer-full` crates
  - Implement `ConfigWatcher` struct: takes config file path, returns a `mpsc::Receiver<ConfigReloadEvent>`
  - Use `notify_debouncer_full::new_debouncer` with 100ms timeout (matching original C# 100ms debounce)
  - On file change: attempt to re-parse config via `Config::from_file()`, send `ConfigReloadEvent::Updated(Config)` on success, `ConfigReloadEvent::Error(String)` on parse failure
  - Provide `ConfigWatcher::start(path) -> Result<(Self, Receiver<ConfigReloadEvent>)>` — spawns watcher thread
  - Provide `ConfigWatcher::stop(self)` — graceful shutdown via drop or explicit call
  - Log reload events via `tracing::info!` / `tracing::warn!`
  - Unit tests: mock file changes by writing to a temp file, verify debounce timing, verify error event on bad TOML

  **Must NOT do**: Apply config changes to running app (that's T14). No direct UI interaction.

  **Recommended Agent Profile**:
  - **Category**: `deep` — **Skills**: [`rust-pro`, `rust-async-patterns`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with T8, T9, T10, T11)
  - **Blocks**: T14 (orchestrator consumes reload events)
  - **Blocked By**: T5 (config parser needed for re-parsing)

  **References**:
  - `src/config.rs` — `Config::from_file()` for re-parsing on change
  - `notify` crate v9 docs — `RecommendedWatcher`, `Config::default().with_poll_interval()`
  - `notify-debouncer-full` — `new_debouncer(Duration, tick_rate, callback)`
  - Original C# `Config.cs` lines 98-115: `FileSystemWatcher` with `ChangeTypes.Changed`, 100ms `_lastRead` debounce

  **Acceptance Criteria**:
  - [ ] `src/watcher.rs` exists with `ConfigWatcher` and `ConfigReloadEvent` enum
  - [ ] Watcher detects file modification and sends Updated event within 200ms
  - [ ] Invalid TOML triggers Error event (not panic)
  - [ ] `cargo test watcher` passes
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Config hot-reload on file change
    Tool: Bash
    Steps:
      1. cargo test watcher::tests::reload_on_change -- --nocapture
      2. Test writes valid TOML to temp file, waits 200ms, checks receiver for Updated event
    Expected: Receiver contains ConfigReloadEvent::Updated with parsed config
    Evidence: .sisyphus/evidence/task-12-hot-reload.txt

  Scenario: Error event on invalid config
    Tool: Bash
    Steps:
      1. cargo test watcher::tests::error_on_bad_toml -- --nocapture
      2. Test writes invalid TOML, waits 200ms, checks receiver for Error event
    Expected: Receiver contains ConfigReloadEvent::Error
    Evidence: .sisyphus/evidence/task-12-bad-toml-error.txt
  ```

  **Commit**: YES
  - Message: `feat(watch): config file watcher with hot-reload`
  - Files: `src/watcher.rs`
  - Pre-commit: `cargo test watcher`

- [ ] 13. Overlay Renderer (egui Painting)

  **What to do**:
  - Create `src/renderer.rs` implementing the main egui paint loop:
    - `OverlayRenderer` struct holds references to: bar states, key configs, layout metrics, fading strips
    - `render(ctx: &egui::Context, state: &AppState)` method paints the full overlay each frame:
      1. Background: fill `egui::Area` with `background_color`
      2. For each key column: draw outline rect, draw key label text (using bundled font), draw press counter
      3. For each active bar in each column: draw filled rect at bar position (color=key color)
      4. For held keys: color uses `Color::pressed()` (golden ratio alpha dimming)
      5. Fading gradient at top: iterate 255 strips, draw semi-transparent rects with decreasing alpha
      6. Request repaint every frame via `ctx.request_repaint()`
  - Use `egui::Painter` API: `rect_filled()`, `rect_stroke()`, `text()`
  - All positions calculated from layout module (T10), all bar data from bar engine (T9)
  - Delta-time from `ctx.input(|i| i.unstable_dt)` for animation updates
  - No business logic — pure rendering of provided state

  **Must NOT do**: Own any state — renderer is stateless, reads from shared AppState. No input handling.

  **Recommended Agent Profile**:
  - **Category**: `deep` — **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES (with T14, T15, T16, T17 — once dependencies met)
  - **Parallel Group**: Wave 3
  - **Blocks**: T14, T18
  - **Blocked By**: T3, T4, T5, T7, T9, T10, T11

  **References**:
  - `src/bars.rs` (T9) — BarState struct, bar positions and sizes
  - `src/layout.rs` (T10) — WindowLayout with column positions and dimensions
  - `src/fading.rs` (T11) — FadingStrip struct and generation
  - `src/font.rs` (T7) — font loading for text rendering
  - `src/types.rs` (T3) — Color with `to_egui()` and `pressed()` methods
  - Original AppWindow.cs `OnUpdate()` — draw loop order: background → outlines → bars → text → fading
  - egui Painter API: `painter.rect_filled(rect, rounding, color)`, `painter.text(pos, anchor, text, font_id, color)`

  **Acceptance Criteria**:
  - [ ] `src/renderer.rs` exists with `OverlayRenderer` struct and `render()` method
  - [ ] `cargo check` succeeds — renderer compiles against real egui types
  - [ ] Renderer draws background, key outlines, bars, text, fading (code review for completeness)
  - [ ] No `unwrap()` or `expect()` in renderer code
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Renderer compiles and integrates with egui types
    Tool: Bash
    Steps:
      1. cargo check 2>&1
      2. Verify no errors in renderer.rs
    Expected: Clean compilation
    Evidence: .sisyphus/evidence/task-13-renderer-check.txt

  Scenario: Renderer code covers all visual elements
    Tool: Bash
    Steps:
      1. grep -c 'rect_filled\|rect_stroke\|text(' src/renderer.rs
      2. Verify >= 3 distinct paint calls (background, bars, text)
    Expected: All visual elements have corresponding paint calls
    Evidence: .sisyphus/evidence/task-13-renderer-coverage.txt
  ```

  **Commit**: YES
  - Message: `feat(renderer): egui overlay renderer with bars, keys, counters`
  - Files: `src/renderer.rs`
  - Pre-commit: `cargo check`

- [ ] 14. App Orchestrator (Main Loop + Threading)

  **What to do**:
  - Create `src/app.rs` with the main application orchestrator `App` struct:
    - `App::new(config: AppConfig) -> Result<Self>` — initializes all subsystems:
      1. Spawn input backend (T8) in a separate thread via `InputBackend::start()` → returns `Receiver<InputEvent>`
      2. Initialize bar engine (T9) with key configs
      3. Compute layout (T10) from config
      4. Generate fading strips (T11)
      5. Start config watcher (T12) → returns `Receiver<ConfigReloadEvent>`
    - `App::run(self) -> Result<()>` — enters the egui_overlay main loop:
      1. Each frame: poll input receiver (non-blocking `try_recv()`), update bar states
      2. Each frame: poll config reload receiver, if Updated → recompute layout, regenerate fading, update bar engine
      3. Each frame: call renderer.render() with current state
      4. Handle graceful shutdown on window close or Ctrl+C
  - Thread architecture: input thread → mpsc → main/render thread ← mpsc ← watcher thread
  - Use `egui_overlay::start()` or equivalent entry point
  - Wire up CLI args (T15) for config path, logging (T16) for file output

  **Must NOT do**: Implement any subsystem logic — only orchestrate existing modules. No `unsafe` code.

  **Recommended Agent Profile**:
  - **Category**: `deep` — **Skills**: [`rust-pro`, `rust-async-patterns`]

  **Parallelization**:
  - **Can Run In Parallel**: NO (integration point, needs most other tasks)
  - **Parallel Group**: Wave 3 (but starts after T13 completes)
  - **Blocks**: T18
  - **Blocked By**: T5, T8, T9, T12, T13, T15, T16, T17

  **References**:
  - `src/input/mod.rs` (T8) — InputBackend trait, `start()` method
  - `src/bars.rs` (T9) — BarEngine::update(), BarEngine::handle_event()
  - `src/layout.rs` (T10) — compute_layout()
  - `src/fading.rs` (T11) — generate_fading_strips()
  - `src/watcher.rs` (T12) — ConfigWatcher::start(), ConfigReloadEvent
  - `src/renderer.rs` (T13) — OverlayRenderer::render()
  - `src/cli.rs` (T15) — CliArgs struct
  - `src/logging.rs` (T16) — init_logging()
  - `src/config.rs` (T17) — ensure_config_exists()
  - Original Program.cs — `Main()` entry: load config → create window → run loop
  - `std::sync::mpsc` — channel(), Sender, Receiver for thread communication

  **Acceptance Criteria**:
  - [ ] `src/app.rs` exists with `App::new()` and `App::run()`
  - [ ] `src/main.rs` wired to: parse CLI → init logging → load config → App::new() → App::run()
  - [ ] Input events flow from input thread to bar engine
  - [ ] Config reload events trigger layout recomputation
  - [ ] `cargo build` succeeds (full binary compiles)
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Application builds and starts without crash
    Tool: Bash
    Steps:
      1. cargo build --release 2>&1
      2. Assert build succeeds (exit code 0)
      3. timeout 5 ./target/release/key-overlay-rs 2>&1 || true
      4. Check stderr for no panic messages
    Expected: Binary builds and runs for 5s without panicking
    Evidence: .sisyphus/evidence/task-14-app-start.txt

  Scenario: Config reload triggers state update
    Tool: Bash
    Steps:
      1. Start app in background: ./target/release/key-overlay-rs &
      2. Wait 1s, modify config.toml (change barSpeed)
      3. Wait 200ms, check log file for reload event
      4. Kill app
    Expected: Log contains 'config reloaded' or similar message
    Evidence: .sisyphus/evidence/task-14-config-reload.txt
  ```

  **Commit**: YES
  - Message: `feat(app): main app orchestrator with threading and event loop`
  - Files: `src/app.rs`, `src/main.rs`
  - Pre-commit: `cargo build`

- [ ] 15. CLI Argument Parsing

  **What to do**:
  - Create `src/cli.rs` using `clap` derive macros:
    ```rust
    #[derive(clap::Parser)]
    #[command(name = "key-overlay-rs", about = "Key press overlay for streaming")]
    pub struct CliArgs {
      #[arg(short, long, default_value = "config.toml")]
      pub config: PathBuf,
      #[arg(short, long, default_value = "false")]
      pub verbose: bool,
    }
    ```
  - `CliArgs::parse()` returns the parsed args
  - Support `--help` and `--version` automatically via clap
  - Wire into `main.rs` (consumed by T14 orchestrator)

  **Must NOT do**: Add subcommands or complex arg logic. Keep minimal.

  **Recommended Agent Profile**:
  - **Category**: `quick` — **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T16, T17)
  - **Blocks**: T14 (consumes CliArgs)
  - **Blocked By**: T2

  **References**:
  - `clap` derive docs: https://docs.rs/clap/latest/clap/_derive/index.html
  - Original has no CLI args — this is a new addition

  **Acceptance Criteria**:
  - [ ] `src/cli.rs` exists with `CliArgs` struct
  - [ ] `cargo run -- --help` shows usage information
  - [ ] `cargo run -- --config custom.toml` accepts path
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: CLI --help works
    Tool: Bash
    Steps:
      1. cargo run -- --help 2>&1
      2. Verify output contains "key-overlay-rs" and "--config"
    Expected: Help text displayed with config flag documented
    Evidence: .sisyphus/evidence/task-15-cli-help.txt
  ```

  **Commit**: YES (groups with T16, T17)
  - Message: `feat(cli): CLI args, file logging, default config generation`
  - Files: `src/cli.rs`
  - Pre-commit: `cargo check`

- [ ] 16. File Logging Setup

  **What to do**:
  - Create `src/logging.rs` using `tracing` + `tracing-subscriber` + `tracing-appender`:
    - `init_logging(verbose: bool) -> Result<WorkerGuard>` function:
      1. Set up file appender writing to `key-overlay-rs.log` in same dir as binary
      2. If verbose: also log to stderr at DEBUG level
      3. Default level: WARN for file, INFO for stderr (if verbose)
      4. Use `tracing_subscriber::fmt::layer()` with JSON or plain text format
    - Return `WorkerGuard` so caller keeps it alive (logs flush on drop)
  - All other modules use `tracing::info!()`, `tracing::warn!()`, `tracing::error!()` macros

  **Must NOT do**: Log to stdout (conflicts with UI). No sensitive data in logs.

  **Recommended Agent Profile**:
  - **Category**: `quick` — **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T15, T17)
  - **Blocks**: T14 (consumes init_logging)
  - **Blocked By**: T2

  **References**:
  - `tracing-appender` docs: `tracing_appender::rolling::daily()` or `non_blocking()`
  - `tracing-subscriber` layered approach: `Registry::default().with(file_layer).with(stderr_layer)`

  **Acceptance Criteria**:
  - [ ] `src/logging.rs` exists with `init_logging()` function
  - [ ] Function returns `WorkerGuard` for flush guarantee
  - [ ] `cargo check` succeeds
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Logging initializes without error
    Tool: Bash
    Steps:
      1. cargo test logging -- --nocapture 2>&1 || cargo check 2>&1
      2. Verify no compilation errors in logging.rs
    Expected: Module compiles cleanly
    Evidence: .sisyphus/evidence/task-16-logging-check.txt
  ```

  **Commit**: YES (groups with T15, T17)
  - Message: `feat(cli): CLI args, file logging, default config generation`
  - Files: `src/logging.rs`
  - Pre-commit: `cargo check`

- [ ] 17. Default Config Generation + First-Run

  **What to do**:
  - Add to `src/config.rs`:
    - `ensure_config_exists(path: &Path) -> Result<bool>` — returns true if created, false if already exists
    - If file doesn't exist: serialize `AppConfig::default()` to TOML and write to path
    - Include helpful comments in generated TOML (use raw string with `# comment` lines above sections)
    - Generated config should have `[general]` section + two `[[key]]` entries (Z and X, like original)
    - Log info message when creating default config
  - Unit test: write to temp dir, verify file contents match expected TOML structure

  **Must NOT do**: Overwrite existing config. No interactive prompts.

  **Recommended Agent Profile**:
  - **Category**: `quick` — **Skills**: [`rust-pro`]

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 3 (with T15, T16)
  - **Blocks**: T14 (called before config load)
  - **Blocked By**: T5 (needs AppConfig::default() and TOML serialization)

  **References**:
  - `src/config.rs` (T5) — AppConfig::default(), TOML format structure
  - Original behavior: no auto-generation (this is a new improvement)
  - `toml::to_string_pretty()` for serialization

  **Acceptance Criteria**:
  - [ ] `ensure_config_exists()` creates valid TOML when file missing
  - [ ] Existing files are NOT overwritten
  - [ ] Generated TOML is parseable by `load_config()` (round-trip test)
  - [ ] `cargo test config_generation` passes
  - [ ] `cargo clippy -- -D warnings` clean

  **QA Scenarios**:
  ```
  Scenario: Default config generation
    Tool: Bash
    Steps:
      1. cargo test config_generation -- --nocapture 2>&1
      2. Verify test creates file in temp dir, reads it back, parses successfully
    Expected: Round-trip test passes — generated TOML produces same AppConfig as default
    Evidence: .sisyphus/evidence/task-17-default-config.txt
  ```

  **Commit**: YES (groups with T15, T16)
  - Message: `feat(cli): CLI args, file logging, default config generation`
  - Files: `src/config.rs` (additions)
  - Pre-commit: `cargo test config`

- [ ] 18. Integration Testing (Mock Input → Render Pipeline)

  **What to do**:
  - Create `tests/integration.rs` with end-to-end pipeline tests using mock input backend
  - Test 1: Construct `AppConfig` → `BarManager` → inject key-down event → tick(dt) → assert bar exists with correct key, position=0, height=min
  - Test 2: Inject key-down, tick N frames, inject key-up → assert bar height grew proportionally to hold duration, bar is now "released" state
  - Test 3: Tick released bar until position exceeds window height → assert bar is removed from active list
  - Test 4: Inject 5 rapid press/release cycles → assert counter == 5 for that key
  - Test 5: Construct `WindowSizer` with 3 keys of varying `_size` → assert calculated width matches formula: `margin + sum(keySize * key._size + outlineThickness * 2 + margin)`
  - Test 6: Load default `AppConfig` → mutate one field → serialize → deserialize → assert round-trip fidelity
  - Test 7: Construct `FadingEffect` → call `generate_strips(height=100)` → assert 255 strips, alpha decreasing 255→0
  - All tests use only public API — no `pub(crate)` test-only backdoors
  - Add `[dev-dependencies]` for `approx` crate (float comparison in bar physics)

  **Must NOT do**:
  - No real window creation — tests are headless (BarManager + state only)
  - No real input capture — use mock `InputBackend` trait impl

  **Recommended Agent Profile**:
  - **Category**: `deep` — multi-module integration tests requiring careful state reasoning
  - **Skills**: [`rust-pro`] — Rust testing patterns, trait mocking, dev-dependencies

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with T19, T20)
  - **Blocks**: F1, F2, F3, F4 (final verification needs tests passing)
  - **Blocked By**: T9 (bar manager), T10 (window sizer), T11 (fading), T8 (input trait), T5 (config)

  **References**:
  - `src/bar.rs` — BarManager API from T9 (new_bar, tick, active_bars)
  - `src/input.rs` — InputBackend trait from T8 (MockInputBackend for testing)
  - `src/layout.rs` — WindowSizer from T10 (calculate_width)
  - `src/fading.rs` — FadingEffect from T11 (generate_strips)
  - `src/config.rs` — AppConfig from T5 (deserialization, defaults)
  - Original C# `Key.cs` lines 42-67 — bar growth/removal logic (behavioral reference)
  - Original C# `AppWindow.cs` lines 18-25 — window width formula

  **Acceptance Criteria**:
  - [ ] `tests/integration.rs` exists with ≥7 test functions
  - [ ] `cargo test --test integration` → all pass
  - [ ] Tests cover: bar creation, bar growth, bar removal, counter, window sizing, config round-trip, fading
  - [ ] No `#[ignore]` annotations on any test
  - [ ] `approx` added to `[dev-dependencies]` in Cargo.toml

  **QA Scenarios**:
  ```
  Scenario: Full integration test suite passes
    Tool: Bash
    Steps:
      1. cargo test --test integration -- --nocapture 2>&1
      2. Verify output contains "test result: ok" with 0 failures
      3. Count test functions: grep -c "#[test]" tests/integration.rs >= 7
    Expected: All 7+ tests pass, zero failures
    Evidence: .sisyphus/evidence/task-18-integration-tests.txt
  ```

  **Commit**: YES (standalone)
  - Message: `test: add integration tests for bar physics, layout, fading, config`
  - Files: `tests/integration.rs`, `Cargo.toml` (dev-deps)
  - Pre-commit: `cargo test --test integration`

- [ ] 19. Cross-Platform Build Verification

  **What to do**:
  - Create `.github/workflows/build.yml` with a CI matrix build:
    - Targets: `ubuntu-latest`, `macos-latest`, `windows-latest`
    - Steps per platform: checkout → install Rust stable → `cargo check` → `cargo clippy --all-targets -- -D warnings` → `cargo test` → `cargo build --release`
    - Linux-specific: install system deps (`libudev-dev`, `libx11-dev`, `libxtst-dev`, `libevdev-dev`) for rdev + evdev compilation
    - macOS-specific: may need Accessibility permissions note (rdev uses CGEvent APIs)
    - Artifact upload: upload release binaries per platform via `actions/upload-artifact@v4`
  - Add build status badge to README.md (placeholder until T20 writes full README)
  - Verify `cargo build --release` locally on Windows before pushing workflow

  **Must NOT do**:
  - No deployment/release publishing — this is build verification only
  - No Docker builds or container targets

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high` — CI/CD YAML with platform-specific knowledge
  - **Skills**: [`github-actions-templates`] — production-ready workflow patterns

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with T18, T20)
  - **Blocks**: F1, F2, F3, F4
  - **Blocked By**: T14 (app must be buildable end-to-end)

  **References**:
  - `Cargo.toml` — full dependency list determines system deps needed per platform
  - `rustdesk-org/rdev` GitHub — check if CI examples exist for Linux deps
  - `evdev` crate docs — Linux build requirements (libudev-dev)
  - GitHub Actions Rust template: `actions-rs/toolchain` or native `rustup` setup
  - AGENTS.md — build/test/lint commands to replicate in CI

  **Acceptance Criteria**:
  - [ ] `.github/workflows/build.yml` exists and is valid YAML
  - [ ] Workflow triggers on push to `main` and on pull requests
  - [ ] Matrix includes all 3 platforms (ubuntu, macos, windows)
  - [ ] Local `cargo build --release` succeeds on Windows
  - [ ] Clippy, test, and build steps defined for each platform
  - [ ] Linux step installs required system dependencies

  **QA Scenarios**:
  ```
  Scenario: Workflow YAML is valid and complete
    Tool: Bash
    Steps:
      1. Verify file exists: test -f .github/workflows/build.yml
      2. Check YAML syntax: python -c "import yaml; yaml.safe_load(open('.github/workflows/build.yml'))" 2>&1 (or use yq)
      3. grep for 'ubuntu-latest' AND 'macos-latest' AND 'windows-latest' in the file
      4. grep for 'cargo clippy' and 'cargo test' and 'cargo build --release'
    Expected: File exists, valid YAML, all 3 platforms present, all build steps present
    Evidence: .sisyphus/evidence/task-19-ci-workflow.txt

  Scenario: Local release build succeeds on Windows
    Tool: Bash
    Steps:
      1. cargo build --release 2>&1
      2. Verify exit code 0
      3. Verify binary exists: test -f target/release/key-overlay-rs.exe
    Expected: Build succeeds, binary exists
    Evidence: .sisyphus/evidence/task-19-release-build.txt
  ```

  **Commit**: YES (standalone)
  - Message: `ci(build): cross-platform build verification workflow`
  - Files: `.github/workflows/build.yml`
  - Pre-commit: `cargo build --release`

- [ ] 20. README + Config Documentation

  **What to do**:
  - Create `README.md` at project root:
    - Project title, single-sentence description ("Transparent keyboard overlay for rhythm games, rewritten in Rust")
    - Feature list matching Must Have (global input capture, scrolling bars, fading, counters, hot-reload, cross-platform)
    - Screenshots placeholder: `<!-- TODO: add screenshot after first working build -->`
    - Installation section: pre-built binary download (placeholder for release URL) + build-from-source instructions for Windows (`cargo build --release`), Linux (`apt install libx11-dev libxtst-dev libevdev-dev && cargo build --release`), macOS (`cargo build --release`)
    - Usage section: `key-overlay-rs [OPTIONS]`, document `--config <path>` and `--generate-config` from T15 CLI
    - Configuration section: brief overview + link to `docs/config.md`
    - Credits: link to original [Friedchicken-42/KeyOverlay](https://github.com/Friedchicken-42/KeyOverlay)
    - License section (MIT, matching Cargo.toml)
  - Create `docs/config.md`:
    - Full TOML config reference: every field from T5 `AppConfig` struct with type, default, valid range, description
    - Group by section: `[window]` (width/height/bar_speed/background/opacity/fading), `[keys]` (array-of-tables with key/size/color/counter_color)
    - Migration guide from original INI format: table mapping old field names → new TOML equivalents
    - Example complete config file (copy from T17 default config generation)
    - Troubleshooting: common issues (permissions on Linux, accessibility on macOS)

  **Must NOT do**:
  - No auto-generated rustdoc API documentation
  - No wiki or external documentation site
  - No marketing copy — keep technical and concise

  **Recommended Agent Profile**:
  - **Category**: `writing`
  - **Skills**: []

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 4 (with Tasks 18, 19)
  - **Blocks**: F1 (plan compliance needs README to verify), F4 (scope fidelity)
  - **Blocked By**: T5 (config struct fields), T14 (app behavior), T15 (CLI args), T17 (default config)

  **References**:
  - `src/config.rs` — `AppConfig`, `WindowConfig`, `KeyConfig` structs (field names, types, defaults) from T5
  - `src/cli.rs` — CLI argument definitions from T15
  - `src/config.rs:generate_default()` or equivalent from T17 — default config TOML content
  - Original config: `https://github.com/Friedchicken-42/KeyOverlay/blob/main/config.ini` — for migration table
  - `Cargo.toml` — license field, package metadata

  **Acceptance Criteria**:
  - [ ] `README.md` exists at project root, non-empty
  - [ ] `docs/config.md` exists, non-empty
  - [ ] README contains: features, install (3 platforms), usage with CLI flags, config link, credits
  - [ ] `docs/config.md` documents every field in `AppConfig` (cross-check with config.rs struct)
  - [ ] Migration table maps all original INI fields to TOML equivalents
  - [ ] Example config in docs matches default generated config from T17

  **QA Scenarios**:
  ```
  Scenario: README completeness check
    Tool: Bash (grep)
    Steps:
      1. grep -c "## Installation" README.md → 1
      2. grep -c "## Usage" README.md → 1
      3. grep -c "## Configuration" README.md → 1
      4. grep -c "Friedchicken-42" README.md → >= 1
      5. grep -c "cargo build" README.md → >= 1
    Expected: All counts match
    Evidence: .sisyphus/evidence/task-20-readme-sections.txt

  Scenario: Config doc covers all fields
    Tool: Bash (grep + diff)
    Steps:
      1. Extract all pub fields from src/config.rs: grep -oP 'pub\s+\w+' src/config.rs | sort > /tmp/config-fields.txt
      2. For each field, grep docs/config.md to confirm it appears
      3. Count fields in config.rs vs documented fields — must be equal
    Expected: 100% field coverage
    Evidence: .sisyphus/evidence/task-20-config-coverage.txt

  Scenario: Migration table present
    Tool: Bash (grep)
    Steps:
      1. grep -c "Migration" docs/config.md → >= 1
      2. grep -c "barSpeed" docs/config.md → >= 1 (original INI field mapped)
      3. grep -c "keySize" docs/config.md → >= 1 (original INI field mapped)
    Expected: Migration section exists with field mappings
    Evidence: .sisyphus/evidence/task-20-migration-table.txt
  ```

  **Commit**: YES (standalone)
  - Message: `docs: README and configuration reference`
  - Files: `README.md`, `docs/config.md`
  - Pre-commit: `cat README.md | head -1`
---

## Final Verification Wave

> 4 review agents run in PARALLEL. ALL must APPROVE. Rejection → fix → re-run.

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Read the plan end-to-end. For each "Must Have": verify implementation exists (read file, run command). For each "Must NOT Have": search codebase for forbidden patterns — reject with file:line if found. Check evidence files exist in .sisyphus/evidence/. Compare deliverables against plan.
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT: APPROVE/REJECT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo clippy --all-targets -- -D warnings` + `cargo test` + `cargo fmt -- --check`. Review all changed files for: unwrap/expect in non-test code, unsafe blocks, empty catch blocks, println! in production paths, unused imports. Check AI slop: excessive comments, over-abstraction, generic names.
  Output: `Build [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [N pass/N fail] | Fmt [PASS/FAIL] | VERDICT`

- [ ] F3. **Real Manual QA** — `unspecified-high`
  Start from clean state (delete config.toml). Run binary. Verify: default config created, overlay window appears, press configured keys and verify bars appear, hold key and verify bar stretches, verify bars scroll upward, verify fading at top, verify counter increments, edit config.toml and verify hot-reload, pass --config custom-path.toml flag.
  Output: `Scenarios [N/N pass] | Integration [N/N] | Edge Cases [N tested] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  For each task: read "What to do", read actual files. Verify 1:1 — everything in spec was built (no missing), nothing beyond spec was built (no creep). Check "Must NOT do" compliance. Detect cross-task contamination: Task N touching Task M's files. Flag unaccounted changes.
  Output: `Tasks [N/N compliant] | Contamination [CLEAN/N issues] | Unaccounted [CLEAN/N files] | VERDICT`

---

## Commit Strategy

- **T1**: `spike(overlay): validate egui_overlay transparency and drawing` — src/main.rs (spike code, will be replaced)
- **T2**: `chore(init): project structure and dependencies` — Cargo.toml, src/lib.rs, src/*/mod.rs
- **T3+T4**: `feat(types): core types, error types, and color parsing with TDD` — src/types.rs, src/color.rs
- **T5+T6**: `feat(config): TOML config parser and key mapping with TDD` — src/config.rs, src/key_map.rs
- **T7**: `chore(assets): bundle JetBrains Mono font` — assets/JetBrainsMono-Bold.ttf, src/font.rs
- **T8**: `feat(input): global input capture backend with rdev` — src/input/mod.rs, src/input/rdev_backend.rs
- **T9**: `feat(bars): bar state machine and physics engine with TDD` — src/bars.rs
- **T10+T11**: `feat(layout): window sizing calculator and fading effect` — src/layout.rs, src/fading.rs
- **T12**: `feat(watch): config file watcher with hot-reload` — src/watcher.rs
- **T13**: `feat(renderer): egui overlay renderer with bars, keys, counters` — src/renderer.rs
- **T14**: `feat(app): main app orchestrator with threading and event loop` — src/app.rs, src/main.rs
- **T15+T16+T17**: `feat(cli): CLI args, file logging, default config generation` — src/cli.rs, src/logging.rs
- **T18**: `test(integration): full pipeline integration tests` — tests/integration.rs
- **T19**: `ci(build): cross-platform build verification` — .github/workflows/build.yml
- **T20**: `docs: README and config documentation` — README.md, docs/config.md

---

## Success Criteria

### Verification Commands
```bash
cargo build --release                            # Expected: success, single binary
cargo test                                        # Expected: all tests pass
cargo clippy --all-targets -- -D warnings         # Expected: 0 warnings
cargo fmt -- --check                              # Expected: no formatting changes
./target/release/key-overlay-rs --help            # Expected: shows CLI usage
./target/release/key-overlay-rs                   # Expected: creates config.toml, shows overlay
```

### Final Checklist
- [ ] All "Must Have" features present and working
- [ ] All "Must NOT Have" items absent from codebase
- [ ] All unit tests pass
- [ ] Clippy clean with -D warnings
- [ ] Binary runs and shows overlay on Windows
- [ ] Config hot-reload works
- [ ] Default config auto-created on first run
- [ ] No unwrap/expect in production code paths