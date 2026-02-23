//! Input backend abstraction and test backend.

use crossbeam_channel::Sender;

use crate::types::{AppError, InputEvent};

/// Input backend interface for platform event sources.
pub trait InputBackend: Send + 'static {
    fn start(&mut self, tx: Sender<InputEvent>) -> Result<(), AppError>;
    fn stop(&mut self) -> Result<(), AppError>;
}

/// Creates the default input backend for the current platform.
pub fn create_backend() -> Box<dyn InputBackend> {
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    {
        Box::new(crate::input::rdev_backend::RdevBackend::new())
    }

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    {
        Box::new(MockBackend::default())
    }
}

/// Deterministic backend for tests that do not need a real device.
#[derive(Debug, Clone, Default)]
pub struct MockBackend {
    scripted_events: Vec<InputEvent>,
    fail_start: Option<String>,
    fail_stop: Option<String>,
    started: bool,
}

impl MockBackend {
    pub fn new(scripted_events: Vec<InputEvent>) -> Self {
        Self {
            scripted_events,
            ..Self::default()
        }
    }

    pub fn with_start_error(mut self, message: impl Into<String>) -> Self {
        self.fail_start = Some(message.into());
        self
    }

    pub fn with_stop_error(mut self, message: impl Into<String>) -> Self {
        self.fail_stop = Some(message.into());
        self
    }

    pub fn is_started(&self) -> bool {
        self.started
    }
}

impl InputBackend for MockBackend {
    fn start(&mut self, tx: Sender<InputEvent>) -> Result<(), AppError> {
        if let Some(message) = self.fail_start.take() {
            return Err(AppError::Input(message));
        }

        for event in self.scripted_events.iter().cloned() {
            tx.send(event)
                .map_err(|err| AppError::Input(format!("failed to send mock event: {err}")))?;
        }

        self.started = true;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), AppError> {
        if let Some(message) = self.fail_stop.take() {
            return Err(AppError::Input(message));
        }

        self.started = false;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crossbeam_channel::unbounded;

    use super::{InputBackend, MockBackend, create_backend};
    use crate::types::{AppError, InputEvent};

    #[test]
    fn test_input_backend_trait_is_object_safe() {
        fn assert_object_safe(_: Box<dyn InputBackend>) {}

        assert_object_safe(Box::new(MockBackend::default()));
    }

    #[test]
    fn test_mock_backend_start_sends_scripted_events() {
        let (tx, rx) = unbounded();
        let mut backend = MockBackend::new(vec![
            InputEvent::KeyPress("A".to_string()),
            InputEvent::MouseRelease("Mouse1".to_string()),
        ]);

        backend
            .start(tx)
            .expect("mock backend start should succeed");

        assert!(backend.is_started());
        assert_eq!(
            rx.try_recv().expect("first event should exist"),
            InputEvent::KeyPress("A".to_string())
        );
        assert_eq!(
            rx.try_recv().expect("second event should exist"),
            InputEvent::MouseRelease("Mouse1".to_string())
        );
    }

    #[test]
    fn test_mock_backend_supports_start_and_stop_failures() {
        let (tx, _) = unbounded();
        let mut backend = MockBackend::default()
            .with_start_error("start failed")
            .with_stop_error("stop failed");

        let start_err = backend
            .start(tx)
            .expect_err("start should fail with configured error");
        assert!(matches!(start_err, AppError::Input(message) if message == "start failed"));

        let stop_err = backend
            .stop()
            .expect_err("stop should fail with configured error");
        assert!(matches!(stop_err, AppError::Input(message) if message == "stop failed"));
    }

    #[test]
    fn test_create_backend_returns_platform_backend() {
        let _backend = create_backend();
    }
}
