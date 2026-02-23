//! Window sizing and layout calculations

/// Layout configuration for overlay
#[derive(Debug, Clone)]
pub struct Layout {
    pub width: u32,
    pub height: u32,
}

pub fn calculate_layout() -> Layout {
    Layout {
        width: 800,
        height: 600,
    }
}
