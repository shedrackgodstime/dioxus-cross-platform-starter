use dioxus::prelude::*;

fn main() {
    // Native clients are not same-origin, so point the server-function
    // transport at the configured API before rendering.
    dioxus::fullstack::set_server_url(core::config::server_api_url());

    dioxus::launch(|| {
        use_context_provider(api::server_fn_fetcher);
        rsx! { ui::App {} }
    });
}
