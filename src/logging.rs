use std::path::Path;
use tracing_appender::rolling::RollingFileAppender;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize logging to either console (default) or daily-rotated files.
///
/// # Arguments
/// * `log_to_file` - If `true`, logs are written to files under `log_dir`
/// * `log_dir` - Directory where logs will be written when file mode is enabled
///
/// # Returns
/// A `WorkerGuard` when file logging is enabled and initialized, otherwise `None`
///
/// # Note
/// If a global default subscriber is already set, this function will still return
/// the guard but the new subscriber won't be set globally. The guard should still
/// be retained to keep the appender alive.
pub fn init_logging(
    log_to_file: bool,
    log_dir: &Path,
) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    if !log_to_file {
        return init_console_logging();
    }

    if let Err(err) = std::fs::create_dir_all(log_dir) {
        eprintln!(
            "Failed to create log directory '{}': {err}. Falling back to console logging.",
            log_dir.display()
        );
        return init_console_logging();
    }

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

    Some(guard)
}

fn init_console_logging() -> Option<tracing_appender::non_blocking::WorkerGuard> {
    let console_layer = fmt::layer()
        .with_target(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_line_number(true);

    let _ = tracing_subscriber::registry()
        .with(console_layer)
        .try_init();
    None
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

        let guard = init_logging(true, &log_dir);

        assert!(guard.is_some());
        assert!(log_dir.exists());
    }

    #[test]
    fn test_init_logging_console_mode_returns_none_guard() {
        let temp_dir = TempDir::new().unwrap();
        let log_dir = temp_dir.path();

        let guard = init_logging(false, log_dir);
        info!("Test message");

        assert!(guard.is_none());
        assert!(fs::read_dir(log_dir).unwrap().next().is_none());
    }
}
