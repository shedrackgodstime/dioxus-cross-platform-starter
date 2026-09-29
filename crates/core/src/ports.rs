use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StorageError {
    #[error("Storage I/O failure: {0}")]
    Io(String),
    #[error("Serialization error: {0}")]
    Serialization(String),
    #[error("Key not found: {0}")]
    NotFound(String),
    #[error("Storage not available on this platform")]
    Unavailable,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NotificationError {
    #[error("Notifications are not available on this platform")]
    Unavailable,
    #[error("Notification permission was denied")]
    PermissionDenied,
    #[error("Notification operation failed: {0}")]
    Failed(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotificationPermission {
    Unknown,
    Granted,
    Denied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationRequest {
    pub title: String,
    pub body: String,
    pub channel: Option<String>,
}

/// Platform adapter for local notifications.
///
/// The core crate defines the contract only. Each launcher can provide a real
/// adapter using its native notification APIs, while unsupported platforms can
/// return `NotificationError::Unavailable` without silently discarding work.
pub trait NotificationService: Send + Sync {
    fn permission(&self) -> Result<NotificationPermission, NotificationError>;
    fn request_permission(&self) -> Result<NotificationPermission, NotificationError>;
    fn send(&self, request: NotificationRequest) -> Result<(), NotificationError>;
}

/// Abstract key-value persistence port.
///
/// Implemented by:
/// - `web-sys::Storage` on Web
/// - Local file/directory store on Desktop (via `directories::ProjectDirs`)
/// - JNI `context.filesDir` on Android
/// - `NSDocumentDirectory` on iOS
pub trait KeyValueStore: Send + Sync {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError>;
    fn set(&self, key: &str, value: &str) -> Result<(), StorageError>;
    fn remove(&self, key: &str) -> Result<(), StorageError>;
    fn clear(&self) -> Result<(), StorageError>;
}

/// Fallback thread-safe in-memory key-value store.
#[derive(Default)]
pub struct MemoryStorage {
    map: std::sync::RwLock<std::collections::HashMap<String, String>>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KeyValueStore for MemoryStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        let guard = self
            .map
            .read()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        Ok(guard.get(key).cloned())
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        let mut guard = self
            .map
            .write()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        guard.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        let mut guard = self
            .map
            .write()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        guard.remove(key);
        Ok(())
    }

    fn clear(&self) -> Result<(), StorageError> {
        let mut guard = self
            .map
            .write()
            .map_err(|e| StorageError::Io(e.to_string()))?;
        guard.clear();
        Ok(())
    }
}

/// Network status reporting port.
pub trait Connectivity: Send + Sync {
    fn is_online(&self) -> bool;
}

/// Device hardware information port.
pub trait DeviceInfo: Send + Sync {
    fn app_version(&self) -> &'static str;
    fn system_name(&self) -> String;
    fn is_touch_device(&self) -> bool;
}
