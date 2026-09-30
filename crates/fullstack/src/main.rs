use dioxus::prelude::*;
#[cfg(feature = "server")]
use std::sync::Arc;

fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        // `router` already mounts the registry that `api`'s server functions
        // join at link time, so `/api/v1/status` is served for the client half.
        // The extra route is a plain probe for infrastructure that just wants a
        // 200 and the bare payload; it shares `api::server_status` so the two
        // cannot drift.
        let router = dioxus::server::router(app).route(
            "/api/v1/health",
            dioxus::server::axum::routing::get(|| async {
                dioxus::server::axum::Json(api::server_status())
            }),
        );
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(|| {
        use_context_provider(api::server_fn_fetcher);
        rsx! { ui::App {} }
    });
}

/// Server-side render root.
#[cfg(feature = "server")]
fn app() -> Element {
    // Answer from the canonical payload directly. Issuing a server-function
    // call while rendering would round-trip to this same listener.
    use_context_provider(local_fetcher);
    rsx! { ui::App {} }
}

#[cfg(feature = "server")]
fn local_fetcher() -> api::StatusFetcher {
    Arc::new(|| {
        let future: api::StatusFuture = Box::pin(async { Ok(api::server_status()) });
        future
    })
}
