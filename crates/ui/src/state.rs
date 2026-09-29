use starter_core::{KeyValueStore, PlatformTarget};
use std::sync::Arc;

/// Global platform context injected by the platform launcher (web, desktop, mobile).
#[derive(Clone)]
pub struct PlatformContext {
    pub target: PlatformTarget,
    pub server_url: String,
    pub storage: Arc<dyn KeyValueStore>,
}

impl PartialEq for PlatformContext {
    fn eq(&self, other: &Self) -> bool {
        self.target == other.target
            && self.server_url == other.server_url
            && Arc::ptr_eq(&self.storage, &other.storage)
    }
}
