use dioxus::prelude::*;

fn main() {
    dioxus::launch(|| {
        // A pure client: it calls the API over HTTP and never serves anything,
        // so the CLI starts no local server and binds no port.
        use_context_provider(api::native_fetcher);
        rsx! { ui::App {} }
    });
}
