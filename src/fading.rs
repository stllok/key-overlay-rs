//! Fading effect for key visualization

/// Represents fading state for visual effects
#[derive(Debug, Clone)]
pub struct FadingEffect {
    pub duration_ms: u32,
}

pub fn create_fading() -> FadingEffect {
    FadingEffect { duration_ms: 200 }
}
