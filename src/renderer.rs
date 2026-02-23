//! egui overlay rendering

use std::time::Duration;

use egui::{
    Align2, Color32, Context, FontData, FontDefinitions, FontFamily, FontId, Frame, Pos2, Rect,
    Stroke,
};
use egui_overlay::EguiOverlay;

use crate::bars::{BarColumn, BarManager};
use crate::fading::calculate_fade_alpha;
use crate::font::load_font;
use crate::layout::{calculate_key_x_positions, calculate_window_width};
use crate::types::{AppConfig, KeyConfig};

const FONT_NAME: &str = "jetbrains-mono";
const KEY_LABEL_SCALE: f32 = 0.32;
const COUNTER_TEXT_SCALE: f32 = 0.24;
const FADE_REGION_RATIO: f32 = 0.25;
const BOTTOM_TEXT_MARGIN: f32 = 8.0;
const KEY_LABEL_VERTICAL_CENTER_RATIO: f32 = 0.6;
const WINDOW_SIZE_EPSILON: f32 = 0.5;

/// Renderer for egui overlay.
#[derive(Debug)]
pub struct Renderer {
    config: AppConfig,
    pub bar_manager: BarManager,
    key_positions: Vec<f32>,
    last_frame_time: Option<f64>,
    font_loaded: bool,
}

impl Renderer {
    pub fn new(config: AppConfig) -> Self {
        let key_positions = calculate_key_x_positions(&config);
        let bar_manager = BarManager::new(config.bar_speed);

        Self {
            config,
            bar_manager,
            key_positions,
            last_frame_time: None,
            font_loaded: false,
        }
    }

    pub fn on_key_press(&mut self, key_name: &str) {
        if let Some((mapped_key, color)) = self
            .config
            .keys
            .iter()
            .find(|key| key.key_name == key_name)
            .map(|key| (key.key_name.clone(), key.color.clone()))
        {
            self.bar_manager.on_key_press(&mapped_key, color);
        }
    }

    pub fn on_key_release(&mut self, key_name: &str) {
        self.bar_manager.on_key_release(key_name);
    }

    pub fn set_config(&mut self, config: AppConfig) {
        self.config = config;
        self.key_positions = calculate_key_x_positions(&self.config);
        self.bar_manager.bar_speed = self.config.bar_speed;
    }

    pub fn desired_window_size(&self) -> [f32; 2] {
        [calculate_window_width(&self.config), self.config.height]
    }

    fn sync_window_size(
        &self,
        glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
        let desired = self.desired_window_size();
        if window_size_needs_update(glfw_backend.window_size_logical, desired) {
            glfw_backend.set_window_size(desired);
        }
    }

    fn ensure_font_loaded(&mut self, egui_context: &Context) {
        if self.font_loaded {
            return;
        }

        let mut font_definitions = FontDefinitions::default();
        font_definitions
            .font_data
            .insert(FONT_NAME.to_string(), FontData::from_static(load_font()));

        for family in [FontFamily::Monospace, FontFamily::Proportional] {
            if let Some(fonts) = font_definitions.families.get_mut(&family) {
                fonts.insert(0, FONT_NAME.to_string());
            }
        }

        egui_context.set_fonts(font_definitions);
        self.font_loaded = true;
    }

    fn update_animation(&mut self, egui_context: &Context) {
        let current_time = egui_context.input(|input| input.time);
        let dt = self
            .last_frame_time
            .map(|last| (current_time - last).max(0.0) as f32)
            .unwrap_or_default()
            .min(0.1);

        self.last_frame_time = Some(current_time);

        self.bar_manager.update(dt);
        self.bar_manager.remove_offscreen(self.config.height);
    }

    fn draw(&self, egui_context: &Context) {
        let frame = Frame::none().fill(self.config.background_color.to_egui());

        egui::CentralPanel::default()
            .frame(frame)
            .show(egui_context, |ui| {
                let canvas = ui.max_rect();
                let painter = ui.painter_at(canvas);
                let fade_height = self.config.height * FADE_REGION_RATIO;

                for (index, key) in self.config.keys.iter().enumerate() {
                    let Some(column_x) = self.key_positions.get(index).copied() else {
                        continue;
                    };

                    let bar_width = self.config.key_size * key.size;
                    let left = canvas.left() + column_x + self.config.outline_thickness;
                    let right = left + bar_width;

                    self.draw_key_anchor_border(&painter, canvas, left, right, key);

                    if let Some(column) = self.bar_manager.columns.get(&key.key_name) {
                        self.draw_column_bars(&painter, canvas, left, right, column, fade_height);
                    }

                    self.draw_key_text(&painter, canvas, left, right, key);
                }
            });
    }

    fn draw_column_bars(
        &self,
        painter: &egui::Painter,
        canvas: Rect,
        left: f32,
        right: f32,
        column: &BarColumn,
        fade_height: f32,
    ) {
        let key_bottom = self.key_bottom(canvas);

        for (bar_index, bar) in column.bars.iter().enumerate() {
            let bottom_y = key_bottom - bar.y_position;
            let top_y = bottom_y - bar.height;

            if bottom_y <= canvas.top() || top_y >= key_bottom {
                continue;
            }

            let draw_top = top_y.max(canvas.top());
            let draw_bottom = bottom_y.min(key_bottom);
            let rect = Rect::from_min_max(Pos2::new(left, draw_top), Pos2::new(right, draw_bottom));

            let is_active_bar = column.is_held && (bar_index + 1 == column.bars.len());
            let base_color = if is_active_bar {
                bar.pressed_color.to_egui()
            } else {
                bar.color.to_egui()
            };

            let fade_alpha = if self.config.fading {
                calculate_fade_alpha(bar.y_position + bar.height, self.config.height, fade_height)
            } else {
                1.0
            };

            let fill_color = with_scaled_alpha(base_color, fade_alpha);
            let stroke_color = with_scaled_alpha(Color32::WHITE, fade_alpha);

            painter.rect_filled(rect, 0.0, fill_color);
            painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(self.config.outline_thickness, stroke_color),
            );
        }
    }

    fn draw_key_text(
        &self,
        painter: &egui::Painter,
        canvas: Rect,
        left: f32,
        right: f32,
        key: &KeyConfig,
    ) {
        let key_bottom = self.key_bottom(canvas);
        let key_top = key_bottom - self.config.key_size;
        let center_x = (left + right) * 0.5;
        let label_pos = Pos2::new(
            center_x,
            key_top + (self.config.key_size * KEY_LABEL_VERTICAL_CENTER_RATIO),
        );
        let label_font = FontId::new(
            (self.config.key_size * KEY_LABEL_SCALE).max(12.0),
            FontFamily::Monospace,
        );

        painter.text(
            label_pos,
            Align2::CENTER_CENTER,
            &key.display_name,
            label_font,
            Color32::WHITE,
        );

        if !self.config.counter {
            return;
        }

        let press_count = self
            .bar_manager
            .columns
            .get(&key.key_name)
            .map_or(0, |column| column.press_count);

        let counter_pos = Pos2::new(center_x, canvas.bottom() - BOTTOM_TEXT_MARGIN);
        let counter_font = FontId::new(
            (self.config.key_size * COUNTER_TEXT_SCALE).max(10.0),
            FontFamily::Monospace,
        );

        painter.text(
            counter_pos,
            Align2::CENTER_BOTTOM,
            format!("{press_count}"),
            counter_font,
            key.color.to_egui(),
        );
    }

    fn draw_key_anchor_border(
        &self,
        painter: &egui::Painter,
        canvas: Rect,
        left: f32,
        right: f32,
        key: &KeyConfig,
    ) {
        let bottom = self.key_bottom(canvas);
        let top = bottom - self.config.key_size;
        let border_rect = Rect::from_min_max(Pos2::new(left, top), Pos2::new(right, bottom));

        painter.rect_stroke(
            border_rect,
            0.0,
            Stroke::new(self.config.outline_thickness, key.color.to_egui()),
        );
    }

    fn key_bottom(&self, canvas: Rect) -> f32 {
        if self.config.counter {
            let counter_font_size = (self.config.key_size * COUNTER_TEXT_SCALE).max(10.0);
            canvas.bottom() - (counter_font_size + (BOTTOM_TEXT_MARGIN * 2.0))
        } else {
            canvas.bottom()
        }
    }
}

impl EguiOverlay for Renderer {
    fn gui_run(
        &mut self,
        egui_context: &Context,
        _default_gfx_backend: &mut egui_overlay::egui_render_three_d::ThreeDBackend,
        glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
        self.sync_window_size(glfw_backend);
        self.ensure_font_loaded(egui_context);
        self.update_animation(egui_context);
        self.draw(egui_context);

        let target_fps = self.config.fps.max(1);
        egui_context.request_repaint_after(Duration::from_secs_f32(1.0 / target_fps as f32));
    }
}

pub fn create_renderer(config: AppConfig) -> Renderer {
    Renderer::new(config)
}

fn with_scaled_alpha(color: Color32, alpha_scale: f32) -> Color32 {
    let scaled = (color.a() as f32 * alpha_scale.clamp(0.0, 1.0))
        .round()
        .clamp(0.0, 255.0) as u8;

    Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), scaled)
}

fn window_size_needs_update(current: [f32; 2], desired: [f32; 2]) -> bool {
    (current[0] - desired[0]).abs() > WINDOW_SIZE_EPSILON
        || (current[1] - desired[1]).abs() > WINDOW_SIZE_EPSILON
}

#[cfg(test)]
mod tests {
    use super::Renderer;
    use crate::types::{AppConfig, Color, KeyConfig};

    const EPSILON: f32 = 1e-6;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn test_desired_window_size_uses_layout_width_and_config_height() {
        let config = AppConfig {
            height: 720.0,
            key_size: 70.0,
            margin: 25.0,
            outline_thickness: 5.0,
            keys: vec![
                KeyConfig {
                    key_name: "Z".to_string(),
                    display_name: "Z".to_string(),
                    color: Color::black(),
                    size: 1.0,
                },
                KeyConfig {
                    key_name: "X".to_string(),
                    display_name: "X".to_string(),
                    color: Color::black(),
                    size: 1.5,
                },
            ],
            ..AppConfig::default()
        };

        let renderer = Renderer::new(config);
        let size = renderer.desired_window_size();

        // width = 25 + (70*1.0 + 10 + 25) + (70*1.5 + 10 + 25) = 270
        assert_f32_eq(size[0], 270.0);
        assert_f32_eq(size[1], 720.0);
    }

    #[test]
    fn test_window_size_needs_update_when_difference_exceeds_epsilon() {
        assert!(super::window_size_needs_update(
            [100.0, 200.0],
            [101.0, 200.0]
        ));
        assert!(super::window_size_needs_update(
            [100.0, 200.0],
            [100.0, 201.0]
        ));
    }

    #[test]
    fn test_window_size_does_not_need_update_within_epsilon() {
        assert!(!super::window_size_needs_update(
            [100.0, 200.0],
            [100.4, 200.0]
        ));
        assert!(!super::window_size_needs_update(
            [100.0, 200.0],
            [100.0, 200.4]
        ));
    }
}
