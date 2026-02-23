//! Configuration file watcher with debounced hot-reload.
//!
//! Uses `notify-debouncer-full` to watch `config.toml` for changes,
//! debounces rapid saves, and invokes a callback with the new [`AppConfig`].

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use notify::RecursiveMode;
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};
use tracing::{info, warn};

use crate::config::load_config;
use crate::types::{AppConfig, AppError};

/// Default debounce timeout in milliseconds.
const DEBOUNCE_TIMEOUT_MS: u64 = 500;

/// Watches a configuration file for changes and invokes a callback on reload.
///
/// Uses a debounced file watcher to avoid reloading on every intermediate
/// write during rapid saves. On each debounced change event, the config
/// file is re-read and parsed; if successful, the callback receives the
/// new [`AppConfig`]. Parse or I/O errors are logged as warnings without
/// crashing.
pub struct ConfigWatcher {
    debouncer: Option<Debouncer<notify::RecommendedWatcher, RecommendedCache>>,
    path: PathBuf,
    callback: Arc<dyn Fn(AppConfig) + Send + Sync>,
}

impl ConfigWatcher {
    /// Creates a new `ConfigWatcher` for the given config file path.
    ///
    /// The `callback` is invoked with a freshly parsed [`AppConfig`] whenever
    /// the watched file is modified (after debouncing).
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Watcher`] if the path cannot be resolved.
    pub fn new(
        path: &Path,
        callback: Box<dyn Fn(AppConfig) + Send + Sync>,
    ) -> Result<Self, AppError> {
        let canonical = path.canonicalize().map_err(|err| {
            AppError::Watcher(format!(
                "failed to canonicalize path '{}': {err}",
                path.display()
            ))
        })?;

        Ok(Self {
            debouncer: None,
            path: canonical,
            // Wrap in Arc so the callback can be shared with the debouncer closure.
            // The Box<dyn Fn + Send> is automatically Send + Sync-safe since Fn is
            // immutably callable from multiple threads.
            callback: Arc::from(Box::leak(callback) as &(dyn Fn(AppConfig) + Send + Sync)),
        })
    }

    /// Starts watching the config file for changes.
    ///
    /// Creates a debounced file watcher that monitors the parent directory
    /// of the config file (to handle atomic saves via rename). Change events
    /// are debounced by 500ms to coalesce rapid saves.
    ///
    /// Calling `start` when already running is a no-op.
    ///
    /// # Errors
    ///
    /// Returns [`AppError::Watcher`] if the watcher cannot be initialized
    /// or if the path cannot be watched.
    pub fn start(&mut self) -> Result<(), AppError> {
        if self.debouncer.is_some() {
            return Ok(());
        }

        let config_path = self.path.clone();
        let callback = Arc::clone(&self.callback);

        let mut debouncer = new_debouncer(
            Duration::from_millis(DEBOUNCE_TIMEOUT_MS),
            None,
            move |result: DebounceEventResult| {
                handle_debounce_event(result, &config_path, &callback);
            },
        )
        .map_err(|err| AppError::Watcher(format!("failed to create debouncer: {err}")))?;

        // Watch the parent directory so editors using atomic saves (write to
        // temp file + rename) are detected correctly.
        let watch_dir = self.path.parent().unwrap_or(Path::new(".")).to_path_buf();

        debouncer
            .watch(&watch_dir, RecursiveMode::NonRecursive)
            .map_err(|err| {
                AppError::Watcher(format!("failed to watch '{}': {err}", watch_dir.display()))
            })?;

        info!("Config watcher started for '{}'", self.path.display());
        self.debouncer = Some(debouncer);
        Ok(())
    }

    /// Stops watching the config file.
    ///
    /// This is a no-op if the watcher is not currently running.
    /// The debouncer is dropped, which stops its internal thread.
    pub fn stop(&mut self) -> Result<(), AppError> {
        if let Some(debouncer) = self.debouncer.take() {
            drop(debouncer);
            info!("Config watcher stopped");
        }
        Ok(())
    }
}

impl Drop for ConfigWatcher {
    fn drop(&mut self) {
        let _ = self.stop();
    }
}

impl std::fmt::Debug for ConfigWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ConfigWatcher")
            .field("path", &self.path)
            .field("running", &self.debouncer.is_some())
            .finish()
    }
}

/// Handles a debounced file system event by reloading config and invoking the callback.
fn handle_debounce_event(
    result: DebounceEventResult,
    config_path: &Path,
    callback: &Arc<dyn Fn(AppConfig) + Send + Sync>,
) {
    match result {
        Ok(events) => {
            let dominated = events.iter().any(|event| {
                matches!(
                    event.kind,
                    notify::EventKind::Modify(_) | notify::EventKind::Create(_)
                )
            });
            if !dominated {
                return;
            }

            info!("Config file changed, reloading...");
            match load_config(config_path) {
                Ok(new_config) => {
                    info!("Config reloaded successfully");
                    callback(new_config);
                }
                Err(err) => {
                    warn!("Failed to reload config: {err}");
                }
            }
        }
        Err(errors) => {
            for error in &errors {
                warn!("File watcher error: {error}");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::thread;
    use std::time::Duration;

    use super::*;

    /// Helper to create a temp config file and return its path.
    fn write_temp_config(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("config.toml");
        fs::write(&path, content).expect("write temp config");
        path
    }

    fn valid_toml() -> &'static str {
        r#"
[general]
height = 700
keySize = 70
barSpeed = 600
backgroundColor = "0,0,0,255"

[[key]]
name = "Z"
color = "255,0,0,255"
"#
    }

    fn modified_toml() -> &'static str {
        r#"
[general]
height = 800
keySize = 80
barSpeed = 700
backgroundColor = "0,0,0,255"

[[key]]
name = "A"
color = "0,255,0,255"
"#
    }

    #[test]
    fn test_watcher_new_creates_instance_for_valid_path() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let watcher = ConfigWatcher::new(&path, Box::new(|_| {}));
        assert!(watcher.is_ok());
    }

    #[test]
    fn test_watcher_new_fails_for_nonexistent_path() {
        let result =
            ConfigWatcher::new(Path::new("/nonexistent/path/config.toml"), Box::new(|_| {}));
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("Watcher error"));
    }

    #[test]
    fn test_watcher_start_and_stop() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let mut watcher = ConfigWatcher::new(&path, Box::new(|_| {})).expect("create watcher");

        assert!(watcher.start().is_ok());
        assert!(watcher.debouncer.is_some());

        assert!(watcher.stop().is_ok());
        assert!(watcher.debouncer.is_none());
    }

    #[test]
    fn test_watcher_start_idempotent() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let mut watcher = ConfigWatcher::new(&path, Box::new(|_| {})).expect("create watcher");

        assert!(watcher.start().is_ok());
        // Second start should be a no-op, not an error.
        assert!(watcher.start().is_ok());
    }

    #[test]
    fn test_watcher_stop_idempotent() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let mut watcher = ConfigWatcher::new(&path, Box::new(|_| {})).expect("create watcher");

        // Stop without start should be fine.
        assert!(watcher.stop().is_ok());
        assert!(watcher.stop().is_ok());
    }

    #[test]
    fn test_watcher_drop_stops_cleanly() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let mut watcher = ConfigWatcher::new(&path, Box::new(|_| {})).expect("create watcher");
        watcher.start().expect("start watcher");

        // Drop should stop the watcher without panicking.
        drop(watcher);
    }

    #[test]
    fn test_watcher_callback_invoked_on_file_change() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let mut watcher = ConfigWatcher::new(
            &path,
            Box::new(move |_config| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }),
        )
        .expect("create watcher");

        watcher.start().expect("start watcher");

        // Give the watcher time to set up.
        thread::sleep(Duration::from_millis(200));

        // Modify the file.
        fs::write(&path, modified_toml()).expect("write modified config");

        // Wait for debounce timeout + processing.
        thread::sleep(Duration::from_millis(1500));

        let count = counter.load(Ordering::SeqCst);
        assert!(
            count >= 1,
            "callback should have been invoked at least once, got {count}"
        );

        watcher.stop().expect("stop watcher");
    }

    #[test]
    fn test_watcher_invalid_config_does_not_crash() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        let mut watcher = ConfigWatcher::new(
            &path,
            Box::new(move |_config| {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }),
        )
        .expect("create watcher");

        watcher.start().expect("start watcher");
        thread::sleep(Duration::from_millis(200));

        // Write invalid TOML - should log warning, not crash.
        fs::write(&path, "{{{{invalid toml").expect("write invalid config");
        thread::sleep(Duration::from_millis(1500));

        // Callback should NOT have been invoked for invalid config.
        let count = counter.load(Ordering::SeqCst);
        assert_eq!(
            count, 0,
            "callback should not be invoked for invalid config"
        );

        watcher.stop().expect("stop watcher");
    }

    #[test]
    fn test_handle_debounce_event_reload_success() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);
        let callback: Arc<dyn Fn(AppConfig) + Send + Sync> = Arc::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Simulate a Modify event.
        let event = notify::Event::new(notify::EventKind::Modify(notify::event::ModifyKind::Data(
            notify::event::DataChange::Content,
        )));
        let debounced =
            notify_debouncer_full::DebouncedEvent::new(event, std::time::Instant::now());
        let result: DebounceEventResult = Ok(vec![debounced]);

        handle_debounce_event(result, &path, &callback);

        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_handle_debounce_event_ignores_non_modify_events() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);
        let callback: Arc<dyn Fn(AppConfig) + Send + Sync> = Arc::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Simulate a Remove event (should be ignored).
        let event = notify::Event::new(notify::EventKind::Remove(notify::event::RemoveKind::File));
        let debounced =
            notify_debouncer_full::DebouncedEvent::new(event, std::time::Instant::now());
        let result: DebounceEventResult = Ok(vec![debounced]);

        handle_debounce_event(result, &path, &callback);

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }

    #[test]
    fn test_handle_debounce_event_handles_errors_gracefully() {
        let dir = tempfile::tempdir().expect("create temp dir");
        let path = write_temp_config(dir.path(), valid_toml());

        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);
        let callback: Arc<dyn Fn(AppConfig) + Send + Sync> = Arc::new(move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Simulate watcher errors.
        let errors = vec![notify::Error::generic("test error")];
        let result: DebounceEventResult = Err(errors);

        // Should not panic.
        handle_debounce_event(result, &path, &callback);

        assert_eq!(counter.load(Ordering::SeqCst), 0);
    }
}
