use dioxus::prelude::*;
use starter_core::{KeyValueStore, MobileOs, PlatformTarget, StorageError};
use starter_ui::{App, PlatformContext};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Mobile persistent key-value store.
/// On Android, stores data in the app's internal filesDir.
/// On iOS, stores data in the app's Sandbox Documents directory.
struct MobileStorage {
    file_path: Option<PathBuf>,
    cache: Mutex<HashMap<String, String>>,
}

impl MobileStorage {
    fn new() -> Self {
        let file_path = Self::resolve_storage_dir().and_then(|base_dir| {
            std::fs::create_dir_all(&base_dir)
                .ok()
                .map(|()| base_dir.join("mobile_storage.json"))
        });

        let cache = if let Some(data) = file_path
            .as_ref()
            .and_then(|path| std::fs::read_to_string(path).ok())
        {
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            HashMap::new()
        };

        Self {
            file_path,
            cache: Mutex::new(cache),
        }
    }

    fn resolve_storage_dir() -> Option<PathBuf> {
        #[cfg(target_os = "android")]
        {
            return Self::android_files_dir();
        }
        #[cfg(target_os = "ios")]
        {
            return directories::ProjectDirs::from("dev", "Dioxus", "Starter")
                .map(|dirs| dirs.data_local_dir().to_path_buf());
        }
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        {
            None
        }
    }

    #[cfg(target_os = "android")]
    fn android_files_dir() -> Option<PathBuf> {
        let ctx = ndk_context::android_context();
        let vm = unsafe { jni::JavaVM::from_raw(ctx.vm().cast()) }.ok()?;
        let mut env = vm.attach_current_thread().ok()?;
        let context = unsafe { jni::objects::JObject::from_raw(ctx.context().cast()) };

        let files_dir_obj = env
            .call_method(&context, "getFilesDir", "()Ljava/io/File;", &[])
            .ok()?
            .l()
            .ok()?;

        let path_jstring = env
            .call_method(
                &files_dir_obj,
                "getAbsolutePath",
                "()Ljava/lang/String;",
                &[],
            )
            .ok()?
            .l()
            .ok()?;

        let path_rust: String = env.get_string(&path_jstring.into()).ok()?.into();
        Some(PathBuf::from(path_rust))
    }

    fn persist(&self) -> Result<(), StorageError> {
        let guard = self
            .cache
            .lock()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        let json = serde_json::to_string_pretty(&*guard)
            .map_err(|e| StorageError::Serialization(e.to_string()))?;
        let file_path = self.file_path.as_ref().ok_or(StorageError::Unavailable)?;
        std::fs::write(file_path, json).map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(())
    }
}

impl KeyValueStore for MobileStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        if self.file_path.is_none() {
            return Err(StorageError::Unavailable);
        }
        let guard = self
            .cache
            .lock()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(guard.get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        if self.file_path.is_none() {
            return Err(StorageError::Unavailable);
        }
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
        if self.file_path.is_none() {
            return Err(StorageError::Unavailable);
        }
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
        if self.file_path.is_none() {
            return Err(StorageError::Unavailable);
        }
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

fn detect_mobile_os() -> MobileOs {
    if cfg!(target_os = "android") {
        MobileOs::Android
    } else if cfg!(target_os = "ios") {
        MobileOs::Ios
    } else {
        MobileOs::Unknown
    }
}

fn main() {
    // 1. Android Logcat initialization (must run before any other logic)
    #[cfg(target_os = "android")]
    {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("dioxus_starter"),
        );
        log::info!("Dioxus Android mobile launcher initialized");
    }

    let storage = Arc::new(MobileStorage::new());
    let os = detect_mobile_os();
    let target = PlatformTarget::Mobile(os);

    // 2. Resolve Server Function endpoint using centralized AppConfig
    let saved_url = storage.get("server_url").ok().flatten();
    let server_url = starter_core::AppConfig::resolve_server_url(
        &target,
        saved_url,
        None,
        option_env!("STARTER_SERVER_URL"),
    );
    let server_url = match starter_core::validate_server_url(&server_url) {
        Ok(url) => url,
        Err(error) => {
            log::error!("invalid API server URL: {error}; using platform default");
            starter_core::AppConfig::resolve_server_url(&target, None, None, None)
        }
    };
    let server_url_static: &'static str = Box::leak(server_url.clone().into_boxed_str());
    dioxus::fullstack::set_server_url(server_url_static);

    let platform = PlatformContext {
        target,
        server_url,
        storage,
    };

    const INDEX_HTML: &str = include_str!("../../../public/index.html");

    let mobile_cfg = dioxus::mobile::Config::new()
        .with_background_color((2, 6, 23, 255))
        .with_custom_index(INDEX_HTML.to_string());

    dioxus::LaunchBuilder::new()
        .with_cfg(mobile_cfg)
        .with_context(platform)
        .launch(App);
}
