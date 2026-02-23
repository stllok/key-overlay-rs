use key_overlay_rs::bars::BarManager;
use key_overlay_rs::config::load_from_str;
use key_overlay_rs::fading::calculate_fade_alpha;
use key_overlay_rs::layout::calculate_window_width;
use key_overlay_rs::types::{AppConfig, Color};

const EPSILON: f32 = 1e-6;

fn assert_f32_eq(actual: f32, expected: f32, message: &str) {
    assert!(
        (actual - expected).abs() < EPSILON,
        "{message}: actual={actual}, expected={expected}"
    );
}

#[test]
fn integration_config_load_from_str_returns_valid_app_config() {
    let toml = r#"
[general]
height = 720
keySize = 64
barSpeed = 420
backgroundColor = "10,20,30,255"
margin = 18
outlineThickness = 4
fading = true
counter = true
fps = 144

[[key]]
name = "A"
color = "255,0,0,255"
size = 1.0

[[key]]
name = "S"
color = "0,255,0,255"
size = 1.5
"#;

    let config = load_from_str(toml).expect("config should parse");

    assert_f32_eq(config.height, 720.0, "height");
    assert_f32_eq(config.key_size, 64.0, "key_size");
    assert_f32_eq(config.bar_speed, 420.0, "bar_speed");
    assert_eq!(
        config.background_color,
        Color::from_rgba_u8(10, 20, 30, 255)
    );
    assert_f32_eq(config.margin, 18.0, "margin");
    assert_f32_eq(config.outline_thickness, 4.0, "outline_thickness");
    assert!(config.fading);
    assert!(config.counter);
    assert_eq!(config.fps, 144);
    assert_eq!(config.keys.len(), 2);
    assert_eq!(config.keys[0].key_name, "A");
    assert_eq!(config.keys[1].key_name, "S");
}

#[test]
fn integration_bar_creation_on_key_press_creates_column_and_bar() {
    let mut manager = BarManager::new(600.0);

    manager.on_key_press("Z", Color::from_rgba_u8(255, 0, 0, 255));

    let column = manager
        .columns
        .get("Z")
        .expect("column should be created for pressed key");
    assert_eq!(column.bars.len(), 1);
    assert_f32_eq(column.bars[0].y_position, 0.0, "new bar y position");
    assert_f32_eq(column.bars[0].height, 1.0, "new bar initial height");
}

#[test]
fn integration_bar_movement_with_delta_time_updates_position_and_growth() {
    let mut manager = BarManager::new(300.0);
    manager.on_key_press("X", Color::from_rgba_u8(0, 255, 255, 255));

    manager.update(0.25);

    let column = manager.columns.get("X").expect("column should exist");
    assert_f32_eq(column.bars[0].y_position, 75.0, "bar y movement");
    assert_f32_eq(column.bars[0].height, 76.0, "held bar growth");
}

#[test]
fn integration_bar_removal_when_offscreen_removes_old_bars() {
    let mut manager = BarManager::new(500.0);
    manager.on_key_press("C", Color::from_rgba_u8(255, 255, 0, 255));

    manager.update(0.5);
    manager.remove_offscreen(100.0);

    let column = manager.columns.get("C").expect("column should exist");
    assert!(column.bars.is_empty());
}

#[test]
fn integration_press_counter_increments_for_each_key_press() {
    let mut manager = BarManager::new(600.0);
    let color = Color::from_rgba_u8(20, 40, 60, 255);

    manager.on_key_press("V", color.clone());
    manager.on_key_release("V");
    manager.on_key_press("V", color);

    let column = manager.columns.get("V").expect("column should exist");
    assert_eq!(column.press_count, 2);
}

#[test]
fn integration_window_width_calculation_matches_formula() {
    let config = AppConfig {
        key_size: 70.0,
        margin: 25.0,
        outline_thickness: 5.0,
        ..AppConfig::default()
    };

    let expected = 25.0 + ((70.0 * 1.0) + (5.0 * 2.0) + 25.0) * 2.0;
    let width = calculate_window_width(&config);

    assert_f32_eq(width, expected, "window width formula");
}

#[test]
fn integration_fading_alpha_calculation_matches_expected_gradient() {
    let window_height = 700.0;
    let fade_height = 140.0;

    let alpha_opaque = calculate_fade_alpha(100.0, window_height, fade_height);
    let alpha_mid = calculate_fade_alpha(630.0, window_height, fade_height);
    let alpha_top = calculate_fade_alpha(700.0, window_height, fade_height);

    assert_f32_eq(alpha_opaque, 1.0, "alpha below fade region");
    assert_f32_eq(alpha_mid, 0.5, "alpha in middle of fade");
    assert_f32_eq(alpha_top, 0.0, "alpha at top of window");
}

#[test]
fn integration_config_round_trip_into_layout_width_uses_loaded_sizes() {
    let toml = r#"
[general]
keySize = 50
margin = 10
outlineThickness = 3

[[key]]
name = "Q"
color = "255,0,0,255"
size = 1.0

[[key]]
name = "W"
color = "0,255,0,255"
size = 2.0
"#;

    let config = load_from_str(toml).expect("config should parse");
    let width = calculate_window_width(&config);
    let expected = 10.0 + ((50.0 * 1.0) + (3.0 * 2.0) + 10.0) + ((50.0 * 2.0) + (3.0 * 2.0) + 10.0);

    assert_f32_eq(width, expected, "loaded config should drive layout width");
}
