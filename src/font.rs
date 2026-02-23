//! Font loading and management
//!
//! Provides access to the bundled JetBrains Mono font. The font is embedded
//! at compile time using `include_bytes!` for zero-dependency deployment.

/// Returns a static reference to the bundled JetBrains Mono Regular font bytes.
///
/// The font is embedded at compile time from `assets/JetBrainsMono-Regular.ttf`
/// and is available as a static byte slice. This function requires no I/O and
/// always returns the same data.
///
/// # Returns
/// A static reference to the TTF font bytes.
///
/// # Example
/// ```no_run
/// let font_bytes = key_overlay_rs::font::load_font();
/// assert!(!font_bytes.is_empty());
/// ```
pub fn load_font() -> &'static [u8] {
    include_bytes!("../assets/JetBrainsMono-Regular.ttf")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_font_returns_non_empty() {
        let font_data = load_font();
        assert!(!font_data.is_empty(), "Font data should not be empty");
    }

    #[test]
    fn test_load_font_is_valid_ttf() {
        let font_data = load_font();
        // TTF files start with the signature 0x00010000 (big-endian) or OTTO for OpenType
        // or just check the first 4 bytes match common TTF/OTF headers
        assert!(font_data.len() >= 4, "Font should be at least 4 bytes");
        let first_four = &font_data[..4];
        // Valid TTF/OTF headers: 0x00010000, OTTO, or true
        let is_ttf = (first_four[0] == 0x00
            && first_four[1] == 0x01
            && first_four[2] == 0x00
            && first_four[3] == 0x00)
            || (first_four == b"OTTO")
            || (first_four == b"true");
        assert!(is_ttf, "Font should have valid TTF/OTF header");
    }

    #[test]
    fn test_load_font_returns_static_reference() {
        let font1 = load_font();
        let font2 = load_font();
        // Both should point to the exact same memory
        assert_eq!(
            font1.as_ptr(),
            font2.as_ptr(),
            "Font should be a static reference"
        );
    }
}
