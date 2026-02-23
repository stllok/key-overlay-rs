//! Input backend trait and abstraction

/// Core trait for input backends
pub trait InputBackend: Send + Sync {
    /// Start listening for keyboard events
    fn start(&mut self) -> anyhow::Result<()>;

    /// Stop listening for keyboard events
    fn stop(&mut self) -> anyhow::Result<()>;
}

/// Generic keyboard event
#[derive(Debug, Clone)]
pub struct KeyboardEvent {
    pub key: String,
}
