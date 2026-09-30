use api::server_status;
use axum::Router;
use dioxus::server::{DioxusRouterExt, FullstackState};
use tower_http::cors::CorsLayer;

fn main() {
    dioxus::serve(|| async {
        // Server functions declared in the `api` crate register themselves at
        // link time, so mounting the registry is what actually serves
        // `/api/v1/status`. A hand-rolled route on that path would shadow the
        // registered function and bypass its encoding entirely.
        //
        // `headless` state runs those functions without rendering pages, which
        // is what keeps this binary API-only.
        let router: Router = Router::new()
            .register_server_functions()
            // Plain probe route for infrastructure that just wants a 200 and
            // the bare payload. Shares `api::server_status` so it cannot drift
            // from the server function above.
            .route(
                "/api/v1/health",
                axum::routing::get(|| async { axum::Json(server_status()) }),
            )
            .with_state(FullstackState::headless())
            // The standalone web client is served from its own dev port, so its
            // requests to this API are cross-origin. Production serves the SPA
            // same-origin and does not depend on this.
            .layer(CorsLayer::permissive());

        Ok(router)
    });
}
