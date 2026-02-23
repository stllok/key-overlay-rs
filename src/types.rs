//! Core domain types for key overlay visualization.

use thiserror::Error;

const GOLDEN_RATIO: f32 = 1.618;

/// RGBA color with normalized f32 channels (0.0 - 1.0).
#[derive(Debug, Clone, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Creates a color from normalized RGBA channels.
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    /// Creates an opaque black color.
    pub fn black() -> Self {
        Self::from_rgba_u8(0, 0, 0, 255)
    }

    /// Returns the pressed-state color using golden-ratio alpha dimming.
    pub fn pressed(&self) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a / GOLDEN_RATIO,
        }
    }

    /// Converts this color to egui's RGBA byte color.
    pub fn to_egui(&self) -> egui::Color32 {
        egui::Color32::from_rgba_unmultiplied(
            (self.r.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.g.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.b.clamp(0.0, 1.0) * 255.0).round() as u8,
            (self.a.clamp(0.0, 1.0) * 255.0).round() as u8,
        )
    }

    /// Creates a color from RGBA bytes.
    pub fn from_rgba_u8(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}

/// Configuration for a single monitored key.
#[derive(Debug, Clone, PartialEq)]
pub struct KeyConfig {
    pub key_name: String,
    pub display_name: String,
    pub color: Color,
    pub size: f32,
}

/// Full application configuration.
#[derive(Debug, Clone, PartialEq)]
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

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            height: 700.0,
            key_size: 70.0,
            bar_speed: 600.0,
            background_color: Color::black(),
            margin: 25.0,
            outline_thickness: 5.0,
            fading: true,
            counter: true,
            fps: 60,
            keys: vec![
                KeyConfig {
                    key_name: "Z".to_string(),
                    display_name: "Z".to_string(),
                    color: Color::from_rgba_u8(255, 0, 0, 255),
                    size: 1.0,
                },
                KeyConfig {
                    key_name: "X".to_string(),
                    display_name: "X".to_string(),
                    color: Color::from_rgba_u8(0, 255, 255, 255),
                    size: 1.0,
                },
            ],
        }
    }
}

/// Application error type.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(String),
    #[error("Input backend error: {0}")]
    Input(String),
    #[error("Render error: {0}")]
    Render(String),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// Represents an input event emitted by input backends.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputEvent {
    KeyPress(String),
    KeyRelease(String),
    MousePress(String),
    MouseRelease(String),
}

#[cfg(test)]
mod tests {
    use super::{AppConfig, Color, KeyConfig};

    const EPSILON: f32 = 1e-6;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "actual={actual}, expected={expected}"
        );
    }

    #[test]
    fn test_color_from_rgba_u8_normalizes_channels() {
        let color = Color::from_rgba_u8(255, 0, 128, 64);

        assert_f32_eq(color.r, 1.0);
        assert_f32_eq(color.g, 0.0);
        assert_f32_eq(color.b, 128.0 / 255.0);
        assert_f32_eq(color.a, 64.0 / 255.0);
    }

    #[test]
    fn test_color_pressed_dims_alpha_by_golden_ratio() {
        let color = Color::new(0.5, 0.25, 0.75, 1.0);
        let pressed = color.pressed();

        assert_f32_eq(pressed.r, color.r);
        assert_f32_eq(pressed.g, color.g);
        assert_f32_eq(pressed.b, color.b);
        assert_f32_eq(pressed.a, 1.0 / 1.618);
    }

    #[test]
    fn test_app_config_default_matches_original_defaults() {
        let config = AppConfig::default();

        assert_f32_eq(config.height, 700.0);
        assert_f32_eq(config.key_size, 70.0);
        assert_f32_eq(config.bar_speed, 600.0);
        assert_eq!(config.background_color, Color::black());
        assert_f32_eq(config.margin, 25.0);
        assert_f32_eq(config.outline_thickness, 5.0);
        assert!(config.fading);
        assert!(config.counter);
        assert_eq!(config.fps, 60);
        assert_eq!(config.keys.len(), 2);

        assert_eq!(config.keys[0].key_name, "Z");
        assert_eq!(config.keys[0].display_name, "Z");
        assert_eq!(config.keys[0].color, Color::from_rgba_u8(255, 0, 0, 255));
        assert_f32_eq(config.keys[0].size, 1.0);

        assert_eq!(config.keys[1].key_name, "X");
        assert_eq!(config.keys[1].display_name, "X");
        assert_eq!(config.keys[1].color, Color::from_rgba_u8(0, 255, 255, 255));
        assert_f32_eq(config.keys[1].size, 1.0);
    }

    #[test]
    fn test_key_config_creation() {
        let key_config = KeyConfig {
            key_name: "Mouse1".to_string(),
            display_name: "M1".to_string(),
            color: Color::from_rgba_u8(10, 20, 30, 200),
            size: 1.25,
        };

        assert_eq!(key_config.key_name, "Mouse1");
        assert_eq!(key_config.display_name, "M1");
        assert_eq!(key_config.color, Color::from_rgba_u8(10, 20, 30, 200));
        assert_f32_eq(key_config.size, 1.25);
    }
}
