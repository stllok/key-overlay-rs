//! Bar state machine for key visualization

/// Represents a single key bar state
#[derive(Debug, Clone)]
pub struct Bar {
    pub key: String,
}

pub fn create_bar(key: String) -> Bar {
    Bar { key }
}
