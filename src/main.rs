#![windows_subsystem = "windows"]

use std::time::{Duration, Instant};

use egui::{Align2, Color32, FontId, Pos2, Rect, Stroke, Vec2};
use egui_overlay::egui_render_three_d::ThreeDBackend as DefaultGfxBackend;
use egui_overlay::EguiOverlay;

const TARGET_FRAME_TIME: Duration = Duration::from_millis(16);

fn main() {
    egui_overlay::start(OverlaySpikeApp::new());
}

struct OverlaySpikeApp {
    started_at: Instant,
    last_frame_at: Instant,
    frame_count: u64,
}

impl OverlaySpikeApp {
    fn new() -> Self {
        let now = Instant::now();
        Self {
            started_at: now,
            last_frame_at: now,
            frame_count: 0,
        }
    }
}

impl EguiOverlay for OverlaySpikeApp {
    fn gui_run(
        &mut self,
        egui_context: &egui::Context,
        _default_gfx_backend: &mut DefaultGfxBackend,
        glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame_at);
        self.last_frame_at = now;
        self.frame_count += 1;

        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.set_passthrough(false);
        } else {
            glfw_backend.set_passthrough(true);
        }

        egui::Area::new("overlay-spike-canvas".into())
            .fixed_pos(Pos2::new(0.0, 0.0))
            .show(egui_context, |ui| {
                let painter = ui.painter();

                let rect = Rect::from_min_size(Pos2::new(120.0, 110.0), Vec2::new(240.0, 120.0));
                painter.rect_filled(
                    rect,
                    6.0,
                    Color32::from_rgba_premultiplied(40, 180, 120, 200),
                );
                painter.rect_stroke(
                    rect,
                    6.0,
                    Stroke::new(3.0, Color32::from_rgba_premultiplied(255, 255, 255, 220)),
                );

                let uptime = now.duration_since(self.started_at).as_secs_f32();
                let fps = if dt.is_zero() {
                    0.0
                } else {
                    1.0 / dt.as_secs_f32()
                };
                let label = format!(
                    "egui_overlay spike\npos=(140,145)\nframes={} fps={fps:.1} uptime={uptime:.1}s",
                    self.frame_count
                );
                painter.text(
                    Pos2::new(140.0, 145.0),
                    Align2::LEFT_TOP,
                    label,
                    FontId::proportional(22.0),
                    Color32::from_rgb(255, 245, 235),
                );
            });

        egui_context.request_repaint_after(TARGET_FRAME_TIME);
    }
}
