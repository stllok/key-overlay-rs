//! Configuration loading and validation from TOML.

use std::path::Path;

use serde::Deserialize;

use crate::color::parse_color;
use crate::types::{AppConfig, AppError, Color, KeyConfig};

/// Raw TOML configuration with optional fields for graceful fallback to defaults.
#[derive(Debug, Default, Deserialize)]
#[serde(default)]
pub struct RawConfig {
    pub general: RawGeneral,
    pub key: Vec<RawKeyConfig>,
}

/// Raw `[general]` TOML section.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RawGeneral {
    pub height: Option<f32>,
    pub key_size: Option<f32>,
    pub bar_speed: Option<f32>,
    pub background_color: Option<String>,
    pub margin: Option<f32>,
    pub outline_thickness: Option<f32>,
    pub fading: Option<bool>,
    pub counter: Option<bool>,
    pub fps: Option<u32>,
}

/// Raw `[[key]]` TOML section.
#[derive(Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RawKeyConfig {
    pub name: Option<String>,
    pub color: Option<String>,
    pub size: Option<f32>,
}

/// Loads and parses configuration from disk.
pub fn load_config(path: &Path) -> Result<AppConfig, AppError> {
    let toml_str = std::fs::read_to_string(path)?;
    load_from_str(&toml_str)
}

/// Loads and parses configuration from TOML text.
pub fn load_from_str(toml_str: &str) -> Result<AppConfig, AppError> {
    let raw: RawConfig = toml::from_str(toml_str)
        .map_err(|err| AppError::Config(format!("failed to parse TOML: {err}")))?;

    let defaults = AppConfig::default();
    let mut config = AppConfig {
        height: raw.general.height.unwrap_or(defaults.height),
        key_size: raw.general.key_size.unwrap_or(defaults.key_size),
        bar_speed: raw.general.bar_speed.unwrap_or(defaults.bar_speed),
        background_color: match raw.general.background_color {
            Some(value) => parse_app_color(&value, "backgroundColor")?,
            None => defaults.background_color,
        },
        margin: raw.general.margin.unwrap_or(defaults.margin),
        outline_thickness: raw
            .general
            .outline_thickness
            .unwrap_or(defaults.outline_thickness),
        fading: raw.general.fading.unwrap_or(defaults.fading),
        counter: raw.general.counter.unwrap_or(defaults.counter),
        fps: raw.general.fps.unwrap_or(defaults.fps),
        keys: if raw.key.is_empty() {
            defaults.keys
        } else {
            parse_raw_keys(raw.key)?
        },
    };

    for warning in validate_config(&config) {
        if warning.contains("bar_speed") {
            config.bar_speed = defaults.bar_speed;
        }
    }

    Ok(config)
}

/// Validates an already-resolved app config and returns non-fatal warnings.
pub fn validate_config(config: &AppConfig) -> Vec<String> {
    let mut warnings = Vec::new();

    if config.bar_speed <= 0.0 {
        warnings.push("bar_speed must be positive; using default 600".to_string());
    }

    if config.keys.is_empty() {
        warnings.push("keys list is empty; using defaults is recommended".to_string());
    }

    warnings
}

fn parse_raw_keys(raw_keys: Vec<RawKeyConfig>) -> Result<Vec<KeyConfig>, AppError> {
    let mut parsed_keys = Vec::with_capacity(raw_keys.len());

    for raw_key in raw_keys {
        let key_name = raw_key
            .name
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| AppError::Config("key entry missing required name".to_string()))?;

        let color = match raw_key.color {
            Some(value) => parse_app_color(&value, "key color")?,
            None => Color::from_rgba_u8(255, 255, 255, 255),
        };

        parsed_keys.push(KeyConfig {
            key_name: key_name.clone(),
            display_name: key_name,
            color,
            size: raw_key.size.unwrap_or(1.0),
        });
    }

    Ok(parsed_keys)
}

fn parse_app_color(raw: &str, field_name: &str) -> Result<Color, AppError> {
    let parsed =
        parse_color(raw).map_err(|err| AppError::Config(format!("invalid {field_name}: {err}")))?;
    Ok(Color::from_rgba_u8(parsed.r, parsed.g, parsed.b, parsed.a))
}

#[cfg(test)]
mod tests {
    use super::{load_from_str, validate_config};
    use crate::types::{AppConfig, Color};

    fn full_valid_toml() -> &'static str {
        r#"
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
"#
    }

    #[test]
    fn test_config_load_from_str_full_valid_toml() {
        let parsed = load_from_str(full_valid_toml()).expect("full config should parse");

        assert_eq!(parsed.height, 700.0);
        assert_eq!(parsed.key_size, 70.0);
        assert_eq!(parsed.bar_speed, 600.0);
        assert_eq!(parsed.background_color, Color::from_rgba_u8(0, 0, 0, 255));
        assert_eq!(parsed.margin, 25.0);
        assert_eq!(parsed.outline_thickness, 5.0);
        assert!(parsed.fading);
        assert!(parsed.counter);
        assert_eq!(parsed.fps, 60);
        assert_eq!(parsed.keys.len(), 2);
        assert_eq!(parsed.keys[0].key_name, "Z");
        assert_eq!(parsed.keys[1].key_name, "X");
    }

    #[test]
    fn test_config_load_from_str_missing_fields_uses_defaults() {
        let parsed =
            load_from_str("[general]\nheight = 820\n").expect("partial config should parse");
        let defaults = AppConfig::default();

        assert_eq!(parsed.height, 820.0);
        assert_eq!(parsed.key_size, defaults.key_size);
        assert_eq!(parsed.bar_speed, defaults.bar_speed);
        assert_eq!(parsed.background_color, defaults.background_color);
        assert_eq!(parsed.keys, defaults.keys);
    }

    #[test]
    fn test_config_load_from_str_invalid_background_color_returns_error() {
        let input = "[general]\nbackgroundColor = \"not-a-color\"\n";

        let err = load_from_str(input).expect_err("invalid color should error");
        assert!(err.to_string().contains("backgroundColor"));
    }

    #[test]
    fn test_config_load_from_str_empty_file_returns_default() {
        let parsed = load_from_str("").expect("empty config should parse as default");

        assert_eq!(parsed, AppConfig::default());
    }

    #[test]
    fn test_config_load_from_str_multiple_keys() {
        let parsed = load_from_str(full_valid_toml()).expect("multiple keys should parse");

        assert_eq!(parsed.keys.len(), 2);
        assert_eq!(parsed.keys[0].display_name, "Z");
        assert_eq!(parsed.keys[1].display_name, "X");
    }

    #[test]
    fn test_config_load_from_str_negative_bar_speed_warns_and_uses_default() {
        let input = r#"
[general]
barSpeed = -25
"#;

        let parsed = load_from_str(input).expect("negative bar speed should not fail parsing");
        assert_eq!(parsed.bar_speed, AppConfig::default().bar_speed);
    }

    #[test]
    fn test_validate_config_negative_bar_speed_reports_warning() {
        let mut config = AppConfig::default();
        config.bar_speed = -10.0;

        let warnings = validate_config(&config);
        assert!(warnings.iter().any(|w| w.contains("bar_speed")));
    }

    #[test]
    fn test_config_load_from_str_missing_key_fields_use_defaults() {
        let input = r#"
[[key]]
name = "C"
"#;

        let parsed = load_from_str(input).expect("partial key should parse");
        assert_eq!(parsed.keys.len(), 1);
        assert_eq!(parsed.keys[0].key_name, "C");
        assert_eq!(parsed.keys[0].display_name, "C");
        assert_eq!(parsed.keys[0].size, 1.0);
        assert_eq!(
            parsed.keys[0].color,
            Color::from_rgba_u8(255, 255, 255, 255)
        );
    }

    #[test]
    fn test_config_load_from_str_invalid_key_color_returns_error() {
        let input = r#"
[[key]]
name = "A"
color = "wrong"
"#;

        let err = load_from_str(input).expect_err("invalid key color should error");
        assert!(err.to_string().contains("key color"));
    }
}
