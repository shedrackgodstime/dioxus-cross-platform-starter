#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
use dioxus::prelude::*;
#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
use starter_ui::App;

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
use starter_core::{KeyValueStore, PlatformTarget, StorageError};
#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
use starter_ui::PlatformContext;
#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
use std::sync::Arc;

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
#[derive(Clone, Default)]
struct WebStorage;

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
fn browser_storage() -> Result<web_sys::Storage, StorageError> {
    web_sys::window()
        .ok_or(StorageError::Unavailable)?
        .local_storage()
        .map_err(|error| StorageError::Io(format!("{error:?}")))?
        .ok_or(StorageError::Unavailable)
}

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
impl KeyValueStore for WebStorage {
    fn get(&self, key: &str) -> Result<Option<String>, StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            browser_storage()?
                .get_item(key)
                .map_err(|error| StorageError::Io(format!("{error:?}")))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = key;
            Ok(None)
        }
    }

    fn set(&self, key: &str, value: &str) -> Result<(), StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            browser_storage()?
                .set_item(key, value)
                .map_err(|error| StorageError::Io(format!("{error:?}")))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = (key, value);
            Ok(())
        }
    }

    fn remove(&self, key: &str) -> Result<(), StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            browser_storage()?
                .remove_item(key)
                .map_err(|error| StorageError::Io(format!("{error:?}")))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = key;
            Ok(())
        }
    }

    fn clear(&self) -> Result<(), StorageError> {
        #[cfg(target_arch = "wasm32")]
        {
            browser_storage()?
                .clear()
                .map_err(|error| StorageError::Io(format!("{error:?}")))
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
    }
}

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
#[component]
fn BrowserApp() -> Element {
    let target = PlatformTarget::Web;
    let server_url = option_env!("STARTER_SERVER_URL")
        .map(str::trim)
        .filter(|url| !url.is_empty());

    #[cfg(target_arch = "wasm32")]
    if let Some(url) = server_url {
        dioxus::fullstack::set_server_url(Box::leak(url.to_string().into_boxed_str()));
    }

    use_context_provider(|| PlatformContext {
        target,
        server_url: server_url.unwrap_or("/").to_string(),
        storage: Arc::new(WebStorage),
    });

    rsx! { App {} }
}

#[cfg(all(not(feature = "server"), target_arch = "wasm32"))]
pub fn launch_client() {
    dioxus::launch(BrowserApp);
}

#[cfg(all(not(feature = "server"), not(target_arch = "wasm32")))]
pub fn launch_client() {}
