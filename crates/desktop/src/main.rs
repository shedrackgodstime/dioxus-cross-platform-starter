use dioxus::desktop::{Config, WindowBuilder};
use dioxus::prelude::*;
use directories::ProjectDirs;
use starter_core::{DesktopOs, KeyValueStore, PlatformTarget, StorageError};
use starter_ui::{App, PlatformContext};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Desktop persistent file-backed JSON key-value store.
struct DesktopStorage {
    file_path: PathBuf,
    cache: Mutex<HashMap<String, String>>,
}

impl DesktopStorage {
    fn new() -> Self {
        let proj = ProjectDirs::from("dev", "dioxus", "starter")
            .map(|p| p.data_dir().to_path_buf())
            .unwrap_or_else(|| PathBuf::from(".dioxus-starter-data"));

        let _ = fs::create_dir_all(&proj);
        let file_path = proj.join("storage.json");

        let cache = if let Ok(data) = fs::read_to_string(&file_path) {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self {
            file_path,
            cache: Mutex::new(cache),
        }
    }

    fn persist(&self) -> Result<(), StorageError> {
        let guard = self
            .cache
            .lock()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        let json = serde_json::to_string_pretty(&*guard)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;
        fs::write(&self.file_path, json).map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(())
    }
}

impl KeyValueStore for DesktopStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let guard = self
            .cache
            .lock()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(guard.get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        {
            let mut guard = self
                .cache
                .lock()
                .map_err(|e| StorageError::Io(e.to_string()))?;
            guard.insert(key.to_string(), value.to_string());
        }
        self.persist()
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        {
            let mut guard = self
                .cache
                .lock()
                .map_err(|e| StorageError::Io(e.to_string()))?;
            guard.remove(key);
        }
        self.persist()
    }

    fn clear(&self) -> Result<(), StorageError> {
        {
            let mut guard = self
                .cache
                .lock()
                .map_err(|e| StorageError::Io(e.to_string()))?;
            guard.clear();
        }
        self.persist()
    }
}

fn detect_desktop_os() -> DesktopOs {
    if cfg!(target_os = "macos") {
        DesktopOs::MacOS
    } else if cfg!(target_os = "windows") {
        DesktopOs::Windows
    } else if cfg!(target_os = "linux") {
        DesktopOs::Linux
    } else {
        DesktopOs::Unknown
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let storage = Arc::new(DesktopStorage::new());
    let os = detect_desktop_os();
    let target = PlatformTarget::Desktop(os);

    // 2. Resolve Server Function endpoint using centralized AppConfig
    let saved_url = storage.get("server_url").ok().flatten();
    let server_url = starter_core::AppConfig::resolve_server_url(
        &target,
        saved_url,
        std::env::var("STARTER_SERVER_URL").ok(),
        option_env!("STARTER_SERVER_URL"),
    );
    let server_url = match starter_core::validate_server_url(&server_url) {
        Ok(url) => url,
        Err(error) => {
            eprintln!("invalid API server URL: {error}; using localhost");
            starter_core::DEFAULT_LOCALHOST_URL.to_string()
        }
    };
    let server_url_static: &'static str = Box::leak(server_url.clone().into_boxed_str());
    dioxus::fullstack::set_server_url(server_url_static);

    let platform = PlatformContext {
        target,
        server_url,
        storage,
    };

    let window_title = format!("Dioxus Starter - {}", platform.target.name());
    let window = WindowBuilder::new()
        .with_title(window_title)
        .with_inner_size(dioxus::desktop::LogicalSize::new(1024.0, 768.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(400.0, 500.0));

    const INDEX_HTML: &str = include_str!("../../../public/index.html");

    let cfg = Config::new()
        .with_window(window)
        .with_background_color((2, 6, 23, 255))
        .with_custom_index(INDEX_HTML.to_string());

    dioxus::LaunchBuilder::new()
        .with_cfg(cfg)
        .with_context(platform)
        .launch(App);
}
