//! Window sizing and layout calculations.
//!
//! Pure functions for calculating overlay window dimensions and key positions.
//! All calculations are deterministic and side-effect-free.
//!
//! # Formula
//!
//! Window width = margin + Σ(column_width for each key)
//! Column width = key_size * size_multiplier + outline_thickness * 2 + margin

use crate::types::AppConfig;

/// Calculate the total window width required to display all keys.
///
/// # Formula
///
/// `width = margin + Σ(key_size * key.size + outline_thickness * 2 + margin)`
///
/// Each key occupies: `(key_size * size_multiplier) + (outline * 2) + margin`
///
/// # Arguments
///
/// * `config` - Application configuration containing key definitions and sizing params
///
/// # Returns
///
/// Total window width in pixels as f32
///
/// # Example
///
/// With 2 keys of size 1.0, key_size=70, margin=25, outline=5:
/// width = 25 + (70+10+25)*2 = 235
pub fn calculate_window_width(config: &AppConfig) -> f32 {
    let mut total = config.margin; // Initial left margin

    for key in &config.keys {
        let column_width = calculate_column_width(
            config.key_size,
            key.size,
            config.outline_thickness,
            config.margin,
        );
        total += column_width;
    }

    total
}

/// Calculate the width of a single key column (key bar + margins + outline).
///
/// # Formula
///
/// `column_width = (key_size * size_multiplier) + (outline_thickness * 2) + margin`
///
/// # Arguments
///
/// * `key_size` - Base width of keys
/// * `size_multiplier` - Key-specific size multiplier (typically 1.0)
/// * `outline_thickness` - Stroke width on both sides
/// * `margin` - Spacing between keys
///
/// # Returns
///
/// Width of a single key column in pixels as f32
pub fn calculate_column_width(
    key_size: f32,
    size_multiplier: f32,
    outline_thickness: f32,
    margin: f32,
) -> f32 {
    (key_size * size_multiplier) + (outline_thickness * 2.0) + margin
}

/// Calculate x-positions for each key in sequence (left to right).
///
/// # Arguments
///
/// * `config` - Application configuration
///
/// # Returns
///
/// Vector of x-coordinates for each key (left edge position)
/// Positions are non-overlapping and in order
pub fn calculate_key_x_positions(config: &AppConfig) -> Vec<f32> {
    let mut positions = Vec::with_capacity(config.keys.len());
    let mut current_x = config.margin;

    for key in &config.keys {
        positions.push(current_x);
        let column_width = calculate_column_width(
            config.key_size,
            key.size,
            config.outline_thickness,
            config.margin,
        );
        current_x += column_width;
    }

    positions
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AppConfig, Color, KeyConfig};

    const EPSILON: f32 = 1e-6;

    fn assert_f32_eq(actual: f32, expected: f32, msg: &str) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "{}: actual={}, expected={}",
            msg,
            actual,
            expected
        );
    }

    #[test]
    fn test_calculate_column_width_single_key() {
        // key_size=70, size=1.0, outline=5, margin=25
        // width = (70*1.0) + (5*2) + 25 = 70 + 10 + 25 = 105
        let width = calculate_column_width(70.0, 1.0, 5.0, 25.0);
        assert_f32_eq(width, 105.0, "column width");
    }

    #[test]
    fn test_calculate_column_width_with_size_multiplier() {
        // key_size=70, size=2.0, outline=5, margin=25
        // width = (70*2.0) + (5*2) + 25 = 140 + 10 + 25 = 175
        let width = calculate_column_width(70.0, 2.0, 5.0, 25.0);
        assert_f32_eq(width, 175.0, "column width with 2x multiplier");
    }

    #[test]
    fn test_calculate_column_width_zero_outline() {
        // key_size=50, size=1.0, outline=0, margin=10
        // width = (50*1.0) + (0*2) + 10 = 50 + 0 + 10 = 60
        let width = calculate_column_width(50.0, 1.0, 0.0, 10.0);
        assert_f32_eq(width, 60.0, "column width with no outline");
    }

    #[test]
    fn test_calculate_window_width_two_keys_default_config() {
        // Test case from spec: 2 keys of size 1.0, key_size=70, margin=25, outline=5
        // width = 25 + (70+10+25)*2 = 25 + 105*2 = 25 + 210 = 235
        let config = AppConfig::default();
        let width = calculate_window_width(&config);
        assert_f32_eq(width, 235.0, "window width for 2 default keys");
    }

    #[test]
    fn test_calculate_window_width_single_key() {
        let config = AppConfig {
            keys: vec![KeyConfig {
                key_name: "Z".to_string(),
                display_name: "Z".to_string(),
                color: Color::from_rgba_u8(255, 0, 0, 255),
                size: 1.0,
            }],
            ..AppConfig::default()
        };

        let width = calculate_window_width(&config);
        // width = 25 + (70+10+25) = 25 + 105 = 130
        assert_f32_eq(width, 130.0, "window width for single key");
    }

    #[test]
    fn test_calculate_window_width_three_keys_mixed_sizes() {
        let config = AppConfig {
            key_size: 70.0,
            margin: 25.0,
            outline_thickness: 5.0,
            keys: vec![
                KeyConfig {
                    key_name: "Z".to_string(),
                    display_name: "Z".to_string(),
                    color: Color::black(),
                    size: 1.0, // width: 105
                },
                KeyConfig {
                    key_name: "X".to_string(),
                    display_name: "X".to_string(),
                    color: Color::black(),
                    size: 1.5, // width: (70*1.5) + 10 + 25 = 140
                },
                KeyConfig {
                    key_name: "C".to_string(),
                    display_name: "C".to_string(),
                    color: Color::black(),
                    size: 2.0, // width: (70*2.0) + 10 + 25 = 175
                },
            ],
            ..AppConfig::default()
        };

        let width = calculate_window_width(&config);
        // width = 25 + 105 + 140 + 175 = 445
        assert_f32_eq(width, 445.0, "window width with 3 mixed-size keys");
    }

    #[test]
    fn test_calculate_window_width_custom_margins_and_outline() {
        let config = AppConfig {
            key_size: 50.0,
            margin: 10.0,
            outline_thickness: 3.0,
            keys: vec![
                KeyConfig {
                    key_name: "A".to_string(),
                    display_name: "A".to_string(),
                    color: Color::black(),
                    size: 1.0, // width: (50*1.0) + (3*2) + 10 = 66
                },
                KeyConfig {
                    key_name: "B".to_string(),
                    display_name: "B".to_string(),
                    color: Color::black(),
                    size: 1.0, // width: 66
                },
            ],
            ..AppConfig::default()
        };

        let width = calculate_window_width(&config);
        // width = 10 + 66 + 66 = 142
        assert_f32_eq(width, 142.0, "window width with custom margins/outline");
    }

    #[test]
    fn test_calculate_key_x_positions_two_keys() {
        let config = AppConfig::default();
        let positions = calculate_key_x_positions(&config);

        assert_eq!(positions.len(), 2, "should have 2 positions");
        assert_f32_eq(positions[0], 25.0, "first key x position");
        // First key width: 105, so second starts at 25 + 105 = 130
        assert_f32_eq(positions[1], 130.0, "second key x position");
    }

    #[test]
    fn test_calculate_key_x_positions_single_key() {
        let config = AppConfig {
            keys: vec![KeyConfig {
                key_name: "Z".to_string(),
                display_name: "Z".to_string(),
                color: Color::black(),
                size: 1.0,
            }],
            ..AppConfig::default()
        };

        let positions = calculate_key_x_positions(&config);

        assert_eq!(positions.len(), 1, "should have 1 position");
        assert_f32_eq(positions[0], 25.0, "single key starts at margin");
    }

    #[test]
    fn test_calculate_key_x_positions_sequential_and_non_overlapping() {
        let config = AppConfig {
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
                    size: 1.0,
                },
                KeyConfig {
                    key_name: "C".to_string(),
                    display_name: "C".to_string(),
                    color: Color::black(),
                    size: 1.0,
                },
            ],
            ..AppConfig::default()
        };

        let positions = calculate_key_x_positions(&config);

        assert_eq!(positions.len(), 3, "should have 3 positions");
        // All keys are size 1.0, so column_width = 70 + 10 + 25 = 105
        assert_f32_eq(positions[0], 25.0, "key 0 at margin");
        assert_f32_eq(positions[1], 130.0, "key 1 at 25 + 105");
        assert_f32_eq(positions[2], 235.0, "key 2 at 130 + 105");

        // Verify non-overlapping: each key's right edge < next key's left edge
        let column_width = 105.0;
        for i in 0..positions.len() - 1 {
            let current_right = positions[i] + column_width;
            let next_left = positions[i + 1];
            assert!(
                current_right <= next_left + EPSILON,
                "keys {} and {} overlap",
                i,
                i + 1
            );
        }
    }

    #[test]
    fn test_calculate_key_x_positions_mixed_sizes() {
        let config = AppConfig {
            key_size: 70.0,
            margin: 25.0,
            outline_thickness: 5.0,
            keys: vec![
                KeyConfig {
                    key_name: "Z".to_string(),
                    display_name: "Z".to_string(),
                    color: Color::black(),
                    size: 1.0, // column_width = 105
                },
                KeyConfig {
                    key_name: "X".to_string(),
                    display_name: "X".to_string(),
                    color: Color::black(),
                    size: 1.5, // column_width = 140
                },
            ],
            ..AppConfig::default()
        };

        let positions = calculate_key_x_positions(&config);

        assert_eq!(positions.len(), 2, "should have 2 positions");
        assert_f32_eq(positions[0], 25.0, "first key starts at margin");
        // First column width = 105, so second starts at 25 + 105 = 130
        assert_f32_eq(positions[1], 130.0, "second key starts after first");
    }

    #[test]
    fn test_calculate_key_x_positions_empty_keys() {
        let config = AppConfig {
            keys: vec![],
            ..AppConfig::default()
        };

        let positions = calculate_key_x_positions(&config);
        assert_eq!(positions.len(), 0, "should have no positions for no keys");
    }
}
