//! Color parsing and manipulation utilities

use serde::{Deserialize, Serialize};

/// Represents a color in RGBA format with u8 components (0-255)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

/// Error type for color parsing failures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorError {
    /// Invalid format or parsing error
    InvalidFormat(String),
    /// Value out of range (0-255)
    OutOfRange(String),
}

impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorError::InvalidFormat(msg) => write!(f, "Invalid color format: {}", msg),
            ColorError::OutOfRange(msg) => write!(f, "Color value out of range: {}", msg),
        }
    }
}

impl std::error::Error for ColorError {}

/// Parse a color string in "R,G,B,A" or "R,G,B" format (u8 values 0-255)
///
/// # Examples
/// ```
/// assert_eq!(parse_color("255,0,128,200"), Ok(Color { r: 255, g: 0, b: 128, a: 200 }));
/// assert_eq!(parse_color("0,0,0"), Ok(Color { r: 0, g: 0, b: 0, a: 255 }));
/// assert!(parse_color("invalid").is_err());
/// ```
pub fn parse_color(s: &str) -> Result<Color, ColorError> {
    let trimmed = s.trim();

    if trimmed.is_empty() {
        return Err(ColorError::InvalidFormat("empty string".to_string()));
    }

    let parts: Vec<&str> = trimmed.split(',').map(|p| p.trim()).collect();

    if parts.len() < 3 || parts.len() > 4 {
        return Err(ColorError::InvalidFormat(format!(
            "expected 3 or 4 components, got {}",
            parts.len()
        )));
    }

    let r = parse_u8_clamped(parts[0])?
        .ok_or_else(|| ColorError::OutOfRange(format!("red: {}", parts[0])))?;
    let g = parse_u8_clamped(parts[1])?
        .ok_or_else(|| ColorError::OutOfRange(format!("green: {}", parts[1])))?;
    let b = parse_u8_clamped(parts[2])?
        .ok_or_else(|| ColorError::OutOfRange(format!("blue: {}", parts[2])))?;

    let a = if parts.len() == 4 {
        parse_u8_clamped(parts[3])?
            .ok_or_else(|| ColorError::OutOfRange(format!("alpha: {}", parts[3])))?
    } else {
        255 // Default to fully opaque
    };

    Ok(Color { r, g, b, a })
}

/// Parse a color string with a default fallback on error
///
/// # Examples
/// ```
/// let default = Color { r: 0, g: 0, b: 0, a: 255 };
/// assert_eq!(parse_color_or_default("255,0,0,255", default), Color { r: 255, g: 0, b: 0, a: 255 });
/// assert_eq!(parse_color_or_default("invalid", default), default);
/// ```
pub fn parse_color_or_default(s: &str, default: Color) -> Color {
    parse_color(s).unwrap_or(default)
}

/// Helper: parse a u8 value, clamping values > 255 to 255, returning None for non-numeric
fn parse_u8_clamped(s: &str) -> Result<Option<u8>, ColorError> {
    let trimmed = s.trim();
    match trimmed.parse::<u32>() {
        Ok(val) => {
            if val > 255 {
                // Clamp to 255
                Ok(Some(255))
            } else {
                Ok(Some(val as u8))
            }
        }
        Err(_) => Err(ColorError::InvalidFormat(format!(
            "not a number: {}",
            trimmed
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_color_valid_rgba() {
        let result = parse_color("255,0,128,200");
        assert_eq!(
            result,
            Ok(Color {
                r: 255,
                g: 0,
                b: 128,
                a: 200
            })
        );
    }

    #[test]
    fn test_parse_color_valid_rgb_defaults_alpha() {
        let result = parse_color("255,0,128");
        assert_eq!(
            result,
            Ok(Color {
                r: 255,
                g: 0,
                b: 128,
                a: 255
            })
        );
    }

    #[test]
    fn test_parse_color_all_zeros() {
        let result = parse_color("0,0,0,0");
        assert_eq!(
            result,
            Ok(Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0
            })
        );
    }

    #[test]
    fn test_parse_color_with_whitespace() {
        let result = parse_color(" 255 , 0 , 128 , 200 ");
        assert_eq!(
            result,
            Ok(Color {
                r: 255,
                g: 0,
                b: 128,
                a: 200
            })
        );
    }

    #[test]
    fn test_parse_color_internal_whitespace() {
        let result = parse_color("  255  ,  0  ,  128  ,  200  ");
        assert_eq!(
            result,
            Ok(Color {
                r: 255,
                g: 0,
                b: 128,
                a: 200
            })
        );
    }

    #[test]
    fn test_parse_color_invalid_format_empty() {
        let result = parse_color("");
        assert!(result.is_err());
        match result {
            Err(ColorError::InvalidFormat(_)) => {}
            _ => panic!("expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_color_invalid_format_too_few_parts() {
        let result = parse_color("255,0");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_invalid_format_too_many_parts() {
        let result = parse_color("255,0,128,200,100");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_invalid_non_numeric() {
        let result = parse_color("not,a,color,255");
        assert!(result.is_err());
        match result {
            Err(ColorError::InvalidFormat(msg)) => assert!(msg.contains("not a number")),
            _ => panic!("expected InvalidFormat error"),
        }
    }

    #[test]
    fn test_parse_color_out_of_range_300() {
        let result = parse_color("300,0,0,0");
        // Should clamp to 255
        assert_eq!(
            result,
            Ok(Color {
                r: 255,
                g: 0,
                b: 0,
                a: 0
            })
        );
    }

    #[test]
    fn test_parse_color_out_of_range_negative() {
        let result = parse_color("-1,0,0,0");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_color_or_default_valid() {
        let default = Color {
            r: 100,
            g: 100,
            b: 100,
            a: 100,
        };
        let result = parse_color_or_default("255,0,0,255", default);
        assert_eq!(
            result,
            Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255
            }
        );
    }

    #[test]
    fn test_parse_color_or_default_invalid() {
        let default = Color {
            r: 100,
            g: 100,
            b: 100,
            a: 100,
        };
        let result = parse_color_or_default("invalid", default);
        assert_eq!(result, default);
    }

    #[test]
    fn test_parse_color_or_default_empty() {
        let default = Color {
            r: 50,
            g: 50,
            b: 50,
            a: 255,
        };
        let result = parse_color_or_default("", default);
        assert_eq!(result, default);
    }

    #[test]
    fn test_color_equality() {
        let c1 = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let c2 = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        assert_eq!(c1, c2);
    }

    #[test]
    fn test_color_inequality() {
        let c1 = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
        let c2 = Color {
            r: 255,
            g: 0,
            b: 0,
            a: 200,
        };
        assert_ne!(c1, c2);
    }
}
