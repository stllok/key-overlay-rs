//! rdev-based input backend for Windows, macOS, and X11

use super::backend::InputBackend;

/// rdev input backend implementation
#[derive(Debug)]
pub struct RdevBackend;

impl InputBackend for RdevBackend {
    fn start(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn create_rdev_backend() -> RdevBackend {
    RdevBackend
}
