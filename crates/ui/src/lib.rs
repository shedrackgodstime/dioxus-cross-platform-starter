pub mod components;
pub mod pages;
pub mod routes;
pub mod state;

use dioxus::prelude::*;
pub use routes::Route;
use starter_core::{MemoryStorage, PlatformTarget, ThemeMode};
pub use state::{PlatformContext, ThemeState};
use std::sync::Arc;

#[component]
pub fn App() -> Element {
    let platform = try_consume_context::<PlatformContext>().unwrap_or_else(|| PlatformContext {
        target: PlatformTarget::Web,
        server_url: "/".to_string(),
        storage: Arc::new(MemoryStorage::new()),
    });

    use_context_provider(|| platform);
    let initial_theme = use_context::<PlatformContext>()
        .storage
        .get("theme_mode")
        .ok()
        .flatten()
        .and_then(|value| ThemeMode::parse(&value))
        .unwrap_or_default();
    let theme = use_signal(|| initial_theme);
    use_context_provider(|| ThemeState { mode: theme });
    let storage = use_context::<PlatformContext>().storage.clone();
    use_effect(move || {
        let mode = theme();
        let _ = storage.set("theme_mode", mode.as_str());
        let script = format!(
            "document.documentElement.dataset.theme = '{}'; document.documentElement.style.colorScheme = '{}';",
            mode.as_str(),
            if matches!(mode, ThemeMode::System) {
                "light dark"
            } else {
                mode.as_str()
            }
        );
        let _ = document::eval(&script);
    });
    rsx! {
        Router::<Route> {}
    }
}
