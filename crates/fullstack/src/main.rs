fn main() {
    #[cfg(feature = "server")]
    dioxus::serve(|| async {
        let router = dioxus::server::router(ui::App).route(
            "/health",
            dioxus::server::axum::routing::get(|| async {
                "{\"status\":\"ok\",\"service\":\"utme-lab-fullstack\"}"
            }),
        );
        Ok(router)
    });

    #[cfg(not(feature = "server"))]
    dioxus::launch(ui::App);
}
