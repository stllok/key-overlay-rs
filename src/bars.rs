//! Bar state machine and delta-time physics.

use std::collections::HashMap;

use crate::types::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct Bar {
    pub y_position: f32,
    pub height: f32,
    pub color: Color,
    pub pressed_color: Color,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BarColumn {
    pub bars: Vec<Bar>,
    pub press_count: u64,
    pub is_held: bool,
    color: Color,
}

impl BarColumn {
    pub fn new(color: Color) -> Self {
        Self {
            bars: Vec::new(),
            press_count: 0,
            is_held: false,
            color,
        }
    }

    pub fn on_key_press(&mut self) {
        self.bars.push(Bar {
            y_position: 0.0,
            height: 1.0,
            color: self.color.clone(),
            pressed_color: self.color.pressed(),
        });
        self.press_count += 1;
        self.is_held = true;
    }

    pub fn on_key_release(&mut self) {
        self.is_held = false;
    }

    pub fn update(&mut self, dt: f32, bar_speed: f32) {
        if dt <= 0.0 {
            return;
        }

        let delta = bar_speed * dt;

        for bar in &mut self.bars {
            bar.y_position += delta;
        }

        if self.is_held
            && let Some(last_bar) = self.bars.last_mut()
        {
            last_bar.height += delta;
        }
    }

    pub fn remove_offscreen(&mut self, window_height: f32) {
        self.bars.retain(|bar| bar.y_position <= window_height);
    }
}

#[derive(Debug, Default)]
pub struct BarManager {
    pub columns: HashMap<String, BarColumn>,
    pub bar_speed: f32,
}

impl BarManager {
    pub fn new(bar_speed: f32) -> Self {
        Self {
            columns: HashMap::new(),
            bar_speed,
        }
    }

    pub fn on_key_press(&mut self, key: &str, color: Color) {
        let column = self
            .columns
            .entry(key.to_string())
            .or_insert_with(|| BarColumn::new(color));
        column.on_key_press();
    }

    pub fn on_key_release(&mut self, key: &str) {
        if let Some(column) = self.columns.get_mut(key) {
            column.on_key_release();
        }
    }

    pub fn update(&mut self, dt: f32) {
        for column in self.columns.values_mut() {
            column.update(dt, self.bar_speed);
        }
    }

    pub fn remove_offscreen(&mut self, window_height: f32) {
        for column in self.columns.values_mut() {
            column.remove_offscreen(window_height);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bar, BarColumn, BarManager};
    use crate::types::Color;

    const EPSILON: f32 = 1e-6;

    fn assert_f32_eq(actual: f32, expected: f32) {
        assert!(
            (actual - expected).abs() < EPSILON,
            "actual={actual}, expected={expected}"
        );
    }

    fn mk_color() -> Color {
        Color::from_rgba_u8(255, 64, 32, 255)
    }

    #[test]
    fn test_bar_column_key_press_creates_new_bar_at_origin() {
        let mut column = BarColumn::new(mk_color());

        column.on_key_press();

        assert_eq!(column.bars.len(), 1);
        let bar = &column.bars[0];
        assert_f32_eq(bar.y_position, 0.0);
        assert_f32_eq(bar.height, 1.0);
    }

    #[test]
    fn test_bar_column_key_press_increments_press_count() {
        let mut column = BarColumn::new(mk_color());

        column.on_key_press();
        column.on_key_press();

        assert_eq!(column.press_count, 2);
    }

    #[test]
    fn test_bar_column_key_press_sets_held_true_and_release_sets_false() {
        let mut column = BarColumn::new(mk_color());

        column.on_key_press();
        assert!(column.is_held);

        column.on_key_release();
        assert!(!column.is_held);
    }

    #[test]
    fn test_bar_column_hold_stretches_last_bar_height_with_delta_time() {
        let mut column = BarColumn::new(mk_color());
        column.on_key_press();

        column.update(0.5, 60.0);

        assert_f32_eq(column.bars[0].height, 31.0);
    }

    #[test]
    fn test_bar_column_update_moves_all_bars_upward_by_speed_times_dt() {
        let mut column = BarColumn::new(mk_color());
        column.bars.push(Bar {
            y_position: 10.0,
            height: 3.0,
            color: mk_color(),
            pressed_color: mk_color().pressed(),
        });
        column.bars.push(Bar {
            y_position: 50.0,
            height: 5.0,
            color: mk_color(),
            pressed_color: mk_color().pressed(),
        });

        column.update(0.25, 100.0);

        assert_f32_eq(column.bars[0].y_position, 35.0);
        assert_f32_eq(column.bars[1].y_position, 75.0);
    }

    #[test]
    fn test_bar_column_update_with_zero_dt_causes_no_movement() {
        let mut column = BarColumn::new(mk_color());
        column.on_key_press();

        column.update(0.0, 1000.0);

        assert_f32_eq(column.bars[0].y_position, 0.0);
        assert_f32_eq(column.bars[0].height, 1.0);
    }

    #[test]
    fn test_bar_column_remove_offscreen_drops_bars_beyond_window_height() {
        let mut column = BarColumn::new(mk_color());
        column.bars.push(Bar {
            y_position: 25.0,
            height: 10.0,
            color: mk_color(),
            pressed_color: mk_color().pressed(),
        });
        column.bars.push(Bar {
            y_position: 120.0,
            height: 5.0,
            color: mk_color(),
            pressed_color: mk_color().pressed(),
        });

        column.remove_offscreen(100.0);

        assert_eq!(column.bars.len(), 1);
        assert_f32_eq(column.bars[0].y_position, 25.0);
    }

    #[test]
    fn test_key_press_assigns_pressed_color_using_color_pressed() {
        let base = mk_color();
        let mut column = BarColumn::new(base.clone());

        column.on_key_press();

        assert_eq!(column.bars[0].color, base);
        assert_eq!(column.bars[0].pressed_color, base.pressed());
    }

    #[test]
    fn test_bar_manager_creates_and_updates_columns_by_key() {
        let mut manager = BarManager::new(200.0);
        let color = mk_color();

        manager.on_key_press("Z", color.clone());
        manager.on_key_press("X", color);
        manager.update(0.5);

        assert_eq!(manager.columns.len(), 2);
        assert_f32_eq(manager.columns["Z"].bars[0].y_position, 100.0);
        assert_f32_eq(manager.columns["X"].bars[0].y_position, 100.0);
    }

    #[test]
    fn test_bar_manager_release_and_remove_offscreen_affects_each_column() {
        let mut manager = BarManager::new(100.0);
        let color = mk_color();

        manager.on_key_press("Z", color.clone());
        manager.on_key_press("X", color.clone());
        manager.on_key_release("Z");
        manager.update(2.0);
        manager.remove_offscreen(150.0);

        assert!(!manager.columns["Z"].is_held);
        assert!(manager.columns["X"].is_held);
        assert!(manager.columns["Z"].bars.is_empty());
        assert!(manager.columns["X"].bars.is_empty());
    }
}
