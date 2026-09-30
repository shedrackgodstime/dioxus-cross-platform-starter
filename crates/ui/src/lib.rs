use dioxus::prelude::*;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

/// Shared application root.
///
/// The launcher must provide an [`api::StatusFetcher`] context before rendering
/// this, via `api::server_fn_fetcher()` or `api::http_fetcher()`.
#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: TAILWIND_CSS }

        main {
            class: "min-h-screen bg-slate-950 p-8 text-white",
            h1 {
                class: "text-3xl font-bold",
                "UTME Lab"
            }
            p {
                class: "mt-2 text-slate-300",
                "Web client is running."
            }
            BackendStatus {}
        }
    }
}

#[component]
fn BackendStatus() -> Element {
    let Some(fetch) = try_use_context::<api::StatusFetcher>() else {
        return rsx! {
            p {
                class: "mt-4 text-slate-400",
                "No API transport was provided by the launcher."
            }
        };
    };

    let mut status = use_signal(|| None::<api::ServerStatus>);
    let mut error = use_signal(|| None::<String>);

    use_effect(move || {
        let fetch = fetch.clone();
        spawn(async move {
            match (fetch)().await {
                Ok(value) => status.set(Some(value)),
                Err(failure) => error.set(Some(failure)),
            }
        });
    });

    match (status(), error()) {
        (_, Some(failure)) => rsx! {
            p { class: "mt-4 text-amber-400", "Backend unavailable: {failure}" }
        },
        (Some(value), None) => rsx! {
            p { class: "mt-4 text-emerald-400", "Backend: {value.status} ({value.service})" }
        },
        (None, None) => rsx! {
            p { class: "mt-4 text-slate-400", "Checking backend..." }
        },
    }
}
