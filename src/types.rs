//! Core domain types for key overlay visualization

use serde::{Deserialize, Serialize};

/// Represents a key event with timestamp and state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyEvent {
    /// Key identifier
    pub key: String,
    /// Timestamp in milliseconds
    pub timestamp: u64,
}

pub fn key_event_new(key: String) -> KeyEvent {
    KeyEvent { key, timestamp: 0 }
}
