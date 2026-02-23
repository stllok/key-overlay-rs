use std::path::Path;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize file logging with daily rotation.
///
/// # Arguments
/// * `log_dir` - Directory where logs will be written
///
/// # Returns
/// A `WorkerGuard` that must be held for logging to remain active
///
/// # Note
/// If a global default subscriber is already set, this function will still return
/// the guard but the new subscriber won't be set globally. The guard should still
/// be retained to keep the appender alive.
pub fn init_logging(log_dir: &Path) -> tracing_appender::non_blocking::WorkerGuard {
    // Create log directory if it doesn't exist
    std::fs::create_dir_all(log_dir).expect("Failed to create log directory");

    // Create daily rolling file appender
    let rolling_appender = RollingFileAppender::new(
        tracing_appender::rolling::Rotation::DAILY,
        log_dir,
        "key-overlay.log",
    );

    // Wrap in non-blocking appender
    let (non_blocking, guard) = tracing_appender::non_blocking(rolling_appender);

    // Configure file layer with formatted output
    let file_layer = fmt::layer()
        .with_writer(non_blocking)
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_line_number(true)
        .with_ansi(false);

    // Try to initialize the global default subscriber
    // If it's already set, ignore the error (common in tests)
    let _ = tracing_subscriber::registry().with(file_layer).try_init();

    guard
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;
    use tracing::info;

    #[test]
    fn test_init_logging_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path().join("logs");

        assert!(!log_dir.exists());

        let _guard = init_logging(&log_dir);

        assert!(log_dir.exists());
    }

    #[test]
    fn test_init_logging_creates_log_file() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path();

        let _guard = init_logging(log_dir);

        info!("Test message");

        // Give non-blocking appender time to flush
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Check that a log file was created with today's date
        let entries = fs::read_dir(log_dir).unwrap();
        let has_log_file = entries
            .filter_map(Result::ok)
            .any(|entry| entry.path().to_string_lossy().contains("key-overlay.log"));

        assert!(has_log_file, "Log file should be created");
    }
}
