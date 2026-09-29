pub mod components;
pub mod pages;
pub mod routes;
pub mod state;

use dioxus::prelude::*;
pub use routes::Route;
use starter_core::{MemoryStorage, PlatformTarget};
pub use state::PlatformContext;
use std::sync::Arc;

#[component]
pub fn App() -> Element {
    let platform = try_consume_context::<PlatformContext>().unwrap_or_else(|| PlatformContext {
        target: PlatformTarget::Web,
        server_url: "/".to_string(),
        storage: Arc::new(MemoryStorage::new()),
    });

    use_context_provider(|| platform);
    rsx! {
        Router::<Route> {}
    }
}
