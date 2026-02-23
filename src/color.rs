//! Color parsing and manipulation utilities

use serde::{Deserialize, Serialize};

/// Represents a color in RGBA format
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub fn parse_color(_hex: &str) -> anyhow::Result<Color> {
    Ok(Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    })
}
