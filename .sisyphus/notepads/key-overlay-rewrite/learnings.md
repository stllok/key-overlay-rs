# Learnings - key-overlay-rewrite

## [2026-02-23T05:59:38Z] Session Start
- Project: Rust rewrite of C# KeyOverlay for osu! streaming
- Primary platform: Windows
- Rendering: egui + egui_overlay
- Input: rdev (rustdesk-org fork) for Win/Mac/X11, evdev for Wayland
- Config: TOML with hot-reload via notify
- Font: Bundled JetBrains Mono (OFL license)

## Architecture Decisions
- Thread model: Input thread → mpsc channel → main/render thread ← mpsc ← watcher thread
- Delta-time based animation (not frame-dependent)
- Graceful error handling: bad config → defaults + warnings, not crash

## [2026-02-23T06:31:00Z] Task 1 - egui_overlay spike learnings
- `egui_overlay` v0.9 starts with `egui_overlay::start(app)` and requires implementing `EguiOverlay::gui_run`.
- In egui 0.29, `Painter::rect_stroke` accepts three args (`rect, corner_radius, stroke`) and does not use `StrokeKind`.
- The crate default startup path sets transparent window + floating (always-on-top) and undecorated window.
- Input passthrough can be demonstrated by toggling `glfw_backend.set_passthrough(...)` based on `egui_context` input demand.
- Smooth frame pacing for spike validation is straightforward with `request_repaint_after(Duration::from_millis(16))`.
