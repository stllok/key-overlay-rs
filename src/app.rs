//! Application orchestrator.

use std::path::{Path, PathBuf};
use std::thread;

use anyhow::{Context as _, Result};
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use egui::Context;
use egui_overlay::EguiOverlay;
use tracing::{error, warn};

use crate::config;
use crate::input;
use crate::renderer::{Renderer, create_renderer};
use crate::types::{AppConfig, InputEvent};
use crate::watcher::ConfigWatcher;

const INPUT_THREAD_NAME: &str = "input-backend";

/// Runs the full application lifecycle.
pub fn run(config_path: &Path) -> Result<()> {
    let config = config::ensure_config_exists(config_path)
        .map_err(anyhow::Error::from)
        .with_context(|| {
            format!(
                "failed to load or create config at '{}'",
                config_path.display()
            )
        })?;

    let log_dir = resolve_log_dir(config_path);
    let _log_guard = crate::logging::init_logging(config.log_to_file, &log_dir);

    let (input_rx, input_shutdown_tx) = start_input_thread()?;
    let (config_rx, mut config_watcher) = start_config_watcher(config_path)?;

    let renderer = create_renderer(config);
    let app = AppOrchestrator::new(renderer, input_rx, config_rx);
    egui_overlay::start(app);

    drop(input_shutdown_tx);
    config_watcher
        .stop()
        .map_err(anyhow::Error::from)
        .context("failed to stop config watcher")?;

    Ok(())
}

fn resolve_log_dir(config_path: &Path) -> PathBuf {
    config_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."))
        .join("logs")
}

fn start_input_thread() -> Result<(Receiver<InputEvent>, Sender<()>)> {
    let mut backend = input::create_backend();
    let (event_tx, event_rx) = unbounded::<InputEvent>();
    let (shutdown_tx, shutdown_rx) = bounded::<()>(1);

    thread::Builder::new()
        .name(INPUT_THREAD_NAME.to_string())
        .spawn(move || run_input_backend(&mut backend, event_tx, shutdown_rx))
        .context("failed to spawn input backend thread")?;

    Ok((event_rx, shutdown_tx))
}

fn run_input_backend(
    backend: &mut Box<dyn input::InputBackend>,
    event_tx: Sender<InputEvent>,
    shutdown_rx: Receiver<()>,
) {
    if let Err(err) = backend.start(event_tx) {
        error!("input backend failed to start: {err}");
        return;
    }

    let _ = shutdown_rx.recv();

    if let Err(err) = backend.stop() {
        warn!("input backend stop failed: {err}");
    }
}

fn start_config_watcher(config_path: &Path) -> Result<(Receiver<AppConfig>, ConfigWatcher)> {
    let (config_tx, config_rx) = unbounded::<AppConfig>();
    let callback = Box::new(move |new_config: AppConfig| {
        if let Err(err) = config_tx.send(new_config) {
            warn!("failed to forward reloaded config: {err}");
        }
    });

    let mut watcher = ConfigWatcher::new(config_path, callback)
        .map_err(anyhow::Error::from)
        .with_context(|| {
            format!(
                "failed to initialize config watcher for '{}'",
                config_path.display()
            )
        })?;
    watcher
        .start()
        .map_err(anyhow::Error::from)
        .context("failed to start config watcher")?;

    Ok((config_rx, watcher))
}

#[derive(Debug)]
struct AppOrchestrator {
    renderer: Renderer,
    input_rx: Receiver<InputEvent>,
    config_rx: Receiver<AppConfig>,
}

impl AppOrchestrator {
    fn new(
        renderer: Renderer,
        input_rx: Receiver<InputEvent>,
        config_rx: Receiver<AppConfig>,
    ) -> Self {
        Self {
            renderer,
            input_rx,
            config_rx,
        }
    }

    fn process_config_updates(&mut self) {
        for config in self.config_rx.try_iter() {
            self.renderer.set_config(config);
        }
    }

    fn process_input_events(&mut self) {
        for event in self.input_rx.try_iter() {
            match event {
                InputEvent::KeyPress(key) | InputEvent::MousePress(key) => {
                    self.renderer.on_key_press(&key);
                }
                InputEvent::KeyRelease(key) | InputEvent::MouseRelease(key) => {
                    self.renderer.on_key_release(&key);
                }
            }
        }
    }
}

impl EguiOverlay for AppOrchestrator {
    fn gui_run(
        &mut self,
        egui_context: &Context,
        default_gfx_backend: &mut egui_overlay::egui_render_three_d::ThreeDBackend,
        glfw_backend: &mut egui_overlay::egui_window_glfw_passthrough::GlfwBackend,
    ) {
        self.process_config_updates();
        self.process_input_events();
        self.renderer
            .gui_run(egui_context, default_gfx_backend, glfw_backend);
    }
}
