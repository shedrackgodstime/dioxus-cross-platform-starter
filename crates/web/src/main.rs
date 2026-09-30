use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        // A SPA reaches the backend over plain HTTP, same-origin by default.
        use_context_provider(api::http_fetcher);
        rsx! { ui::App {} }
    });
}
