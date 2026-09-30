use api::ServerStatus;

fn main() {
    dioxus::serve(|| async {
        let status = || async {
            dioxus::server::axum::Json(ServerStatus {
                status: "ok".to_string(),
                service: "utme-lab-server".to_string(),
            })
        };
        let router = dioxus::server::axum::Router::new()
            .route("/api/v1/health", dioxus::server::axum::routing::get(status))
            .route("/api/v1/status", dioxus::server::axum::routing::get(status));
        Ok(router)
    });
}
