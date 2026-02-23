//! Input backend abstraction for keyboard event capture

pub mod backend;
pub mod key_mapping;
pub mod rdev_backend;

pub use backend::InputBackend;
pub use key_mapping::KeyId;
pub use rdev_backend::RdevBackend;
