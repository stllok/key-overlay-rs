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
