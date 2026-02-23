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
use crate::layout::calculate_key_x_positions;
use crate::types::{AppConfig, KeyConfig};

const FONT_NAME: &str = "jetbrains-mono";
const KEY_LABEL_SCALE: f32 = 0.32;
const COUNTER_TEXT_SCALE: f32 = 0.24;
const FADE_REGION_RATIO: f32 = 0.25;

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

    fn ensure_font_loaded(&mut self, egui_context: &Context) {
        if self.font_loaded {
            return;
        }

        let mut font_definitions = FontDefinitions::default();
        font_definitions.font_data.insert(
            FONT_NAME.to_string(),
            FontData::from_static(load_font()),
        );

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
        for (bar_index, bar) in column.bars.iter().enumerate() {
            let bottom_y = canvas.bottom() - bar.y_position;
            let top_y = bottom_y - bar.height;

            if bottom_y <= canvas.top() || top_y >= canvas.bottom() {
                continue;
            }

            let draw_top = top_y.max(canvas.top());
            let draw_bottom = bottom_y.min(canvas.bottom());
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
        let center_x = (left + right) * 0.5;
        let label_pos = Pos2::new(center_x, canvas.bottom() - 8.0);
        let label_font = FontId::new(
            (self.config.key_size * KEY_LABEL_SCALE).max(12.0),
            FontFamily::Monospace,
        );

        painter.text(
            label_pos,
            Align2::CENTER_BOTTOM,
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

        let counter_pos = Pos2::new(center_x, canvas.bottom() - self.config.key_size - 16.0);
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
}

impl EguiOverlay for Renderer {
    fn gui_run(
        &mut self,
        egui_context: &Context,
        _default_gfx_backend: &mut egui_overlay::egui_render_three_d::ThreeDBackend,
        _glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
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
