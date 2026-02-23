//! Input backend abstraction for keyboard event capture

pub mod backend;
pub mod rdev_backend;

pub use backend::InputBackend;
pub use rdev_backend::RdevBackend;
