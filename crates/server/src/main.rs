fn main() {
    dioxus::serve(|| async {
        let router = dioxus::server::axum::Router::new().route(
            "/health",
            dioxus::server::axum::routing::get(|| async {
                "{\"status\":\"ok\",\"service\":\"utme-lab-server\"}"
            }),
        );
        Ok(router)
    });
}
