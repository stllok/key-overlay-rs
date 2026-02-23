//! rdev-based input backend for Windows, macOS, and X11.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

use crossbeam_channel::Sender;
use rdev::EventType;

use crate::input::backend::InputBackend;
use crate::input::key_mapping::KeyId;
use crate::types::{AppError, InputEvent};

const LISTENER_THREAD_NAME: &str = "rdev-input-listener";

/// `rdev` input backend implementation.
#[derive(Debug)]
pub struct RdevBackend {
    running: Arc<AtomicBool>,
    listener_thread: Option<JoinHandle<()>>,
}

impl Default for RdevBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl RdevBackend {
    pub fn new() -> Self {
        Self {
            running: Arc::new(AtomicBool::new(false)),
            listener_thread: None,
        }
    }

    fn cleanup_finished_listener_thread(&mut self) {
        let is_finished = self
            .listener_thread
            .as_ref()
            .is_some_and(JoinHandle::is_finished);
        if is_finished {
            let _ = self
                .listener_thread
                .take()
                .expect("listener thread exists when marked finished")
                .join();
        }
    }
}

impl InputBackend for RdevBackend {
    fn start(&mut self, tx: Sender<InputEvent>) -> Result<(), AppError> {
        self.cleanup_finished_listener_thread();

        if self.running.load(Ordering::SeqCst) {
            return Err(AppError::Input(
                "rdev backend is already running".to_string(),
            ));
        }

        if self.listener_thread.is_some() {
            return Err(AppError::Input(
                "rdev backend listener thread is still active".to_string(),
            ));
        }

        self.running.store(true, Ordering::SeqCst);

        let running = Arc::clone(&self.running);
        let listener_tx = tx;
        let builder = thread::Builder::new().name(LISTENER_THREAD_NAME.to_string());

        let handle = builder
            .spawn(move || {
                let callback_running = Arc::clone(&running);
                let callback = move |event: rdev::Event| {
                    if !callback_running.load(Ordering::Relaxed) {
                        return;
                    }

                    if let Some(input_event) = map_rdev_event_to_input_event(event.event_type)
                        && listener_tx.send(input_event).is_err()
                    {
                        callback_running.store(false, Ordering::SeqCst);
                    }
                };

                if let Err(err) = rdev::listen(callback) {
                    tracing::error!("rdev listen loop stopped: {err:#?}");
                    running.store(false, Ordering::SeqCst);
                }
            })
            .map_err(|err| {
                self.running.store(false, Ordering::SeqCst);
                AppError::Input(format!("failed to spawn rdev listener thread: {err}"))
            })?;

        self.listener_thread = Some(handle);

        Ok(())
    }

    fn stop(&mut self) -> Result<(), AppError> {
        self.running.store(false, Ordering::SeqCst);
        self.cleanup_finished_listener_thread();
        Ok(())
    }
}

/// Maps rdev `EventType` to `InputEvent`.
fn map_rdev_event_to_input_event(event: EventType) -> Option<InputEvent> {
    match event {
        EventType::KeyPress(key) => {
            let key_id = KeyId::try_from(key).ok()?;
            Some(InputEvent::KeyPress(key_id.to_string()))
        }
        EventType::KeyRelease(key) => {
            let key_id = KeyId::try_from(key).ok()?;
            Some(InputEvent::KeyRelease(key_id.to_string()))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rdev_backend_new() {
        let backend = RdevBackend::new();
        assert!(!backend.running.load(Ordering::SeqCst));
        assert!(backend.listener_thread.is_none());
    }

    #[test]
    fn test_rdev_backend_default() {
        let backend = RdevBackend::default();
        assert!(!backend.running.load(Ordering::SeqCst));
        assert!(backend.listener_thread.is_none());
    }
}
