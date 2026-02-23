//! Fading effect generator for the overlay.
//!
//! Provides linear alpha fade from opaque at the bottom to transparent at the top,
//! used by the renderer to apply a fade effect to the key display area.

/// Calculates the alpha value (0.0-1.0) for a given position in the fade region.
///
/// The fade effect creates a linear gradient:
/// - At the bottom of the window (y=0): alpha = 1.0 (fully opaque)
/// - At the start of fade region (y=window_height-fade_height): alpha = 1.0
/// - At the top of fade region (y=window_height): alpha = 0.0 (fully transparent)
/// - Beyond the top: alpha = 0.0
///
/// # Arguments
/// * `y_position` - Y coordinate in the window (0.0 at bottom, increases upward)
/// * `window_height` - Total height of the window
/// * `fade_height` - Height of the fade region from the top
///
/// # Returns
/// Alpha value between 0.0 (transparent) and 1.0 (opaque)
pub fn calculate_fade_alpha(
    y_position: f32,
    window_height: f32,
    fade_height: f32,
) -> f32 {
    // No fade region or invalid params: fully opaque below top
    if fade_height <= 0.0 {
        return if y_position < window_height { 1.0 } else { 0.0 };
    }
    // Below the fade region: fully opaque
    if y_position <= window_height - fade_height {
        return 1.0;
    }
    // Above the window: fully transparent
    if y_position >= window_height {
        return 0.0;
    }
    // Within the fade region: linear interpolation
    let fade_start = window_height - fade_height;
    let distance_from_start = y_position - fade_start;
    1.0 - (distance_from_start / fade_height)
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn test_fade_alpha_at_bottom_is_opaque() {
        let alpha = calculate_fade_alpha(0.0, 800.0, 100.0);
        assert_f32_eq(alpha, 1.0, "Bottom of window should be fully opaque");
    }

    #[test]
    fn test_fade_alpha_at_fade_start_is_opaque() {
        let window_height = 800.0;
        let fade_height = 100.0;
        let fade_start = window_height - fade_height; // 700.0
        let alpha = calculate_fade_alpha(fade_start, window_height, fade_height);
        assert_f32_eq(alpha, 1.0, "At fade region start should be fully opaque");
    }

    #[test]
    fn test_fade_alpha_at_top_is_transparent() {
        let alpha = calculate_fade_alpha(800.0, 800.0, 100.0);
        assert_f32_eq(alpha, 0.0, "Top of window should be fully transparent");
    }

    #[test]
    fn test_fade_alpha_beyond_top_is_transparent() {
        let alpha = calculate_fade_alpha(900.0, 800.0, 100.0);
        assert_f32_eq(
            alpha,
            0.0,
            "Beyond top of window should be fully transparent",
        );
    }

    #[test]
    fn test_fade_alpha_at_midpoint() {
        let window_height = 800.0;
        let fade_height = 100.0;
        let fade_start = 700.0;
        let midpoint = fade_start + fade_height / 2.0; // 750.0
        let alpha = calculate_fade_alpha(midpoint, window_height, fade_height);
        assert_f32_eq(alpha, 0.5, "Midpoint of fade should be 0.5 alpha");
    }

    #[test]
    fn test_fade_alpha_linear_interpolation() {
        let window_height = 1000.0;
        let fade_height = 200.0;
        let fade_start = 800.0;

        // Test quarter point (25% into fade region)
        let quarter = fade_start + fade_height / 4.0;
        let alpha_quarter = calculate_fade_alpha(quarter, window_height, fade_height);
        assert_f32_eq(alpha_quarter, 0.75, "25% into fade: alpha should be 0.75");

        // Test three-quarter point (75% into fade region)
        let three_quarter = fade_start + (3.0 * fade_height / 4.0);
        let alpha_three = calculate_fade_alpha(three_quarter, window_height, fade_height);
        assert_f32_eq(alpha_three, 0.25, "75% into fade: alpha should be 0.25");
    }

    #[test]
    fn test_fade_alpha_with_negative_y_position() {
        let alpha = calculate_fade_alpha(-100.0, 800.0, 100.0);
        assert_f32_eq(alpha, 1.0, "Negative y position should be fully opaque");
    }

    #[test]
    fn test_fade_alpha_with_zero_fade_height() {
        // Edge case: no fade region
        let alpha_below = calculate_fade_alpha(700.0, 800.0, 0.0);
        assert_f32_eq(
            alpha_below,
            1.0,
            "Below fade region (no fade): should be opaque",
        );

        let alpha_at_top = calculate_fade_alpha(800.0, 800.0, 0.0);
        assert_f32_eq(
            alpha_at_top,
            0.0,
            "At top with zero fade: should be transparent",
        );
    }

    #[test]
    fn test_fade_alpha_with_fade_covering_entire_window() {
        let window_height = 800.0;
        let fade_height = 800.0; // Fade covers entire window
        let alpha_bottom = calculate_fade_alpha(0.0, window_height, fade_height);
        let alpha_top = calculate_fade_alpha(800.0, window_height, fade_height);

        assert_f32_eq(alpha_bottom, 1.0, "Bottom should be opaque");
        assert_f32_eq(alpha_top, 0.0, "Top should be transparent");
    }
}
