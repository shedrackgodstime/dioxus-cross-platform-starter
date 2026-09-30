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
    // A fullstack server render already knows the answer synchronously, so it
    // seeds the status and the server-rendered HTML shows a real value instead
    // of a loading line that only resolves after hydration.
    let pre_resolved = try_use_context::<api::ServerStatus>();
    let fetch = try_use_context::<api::StatusFetcher>();
    let has_transport = fetch.is_some();

    let mut status = use_signal(|| pre_resolved);
    let mut error = use_signal(|| None::<String>);

    // Cloned so the flag above stays readable in the render below; every hook
    // still runs unconditionally.
    let fetch_for_effect = fetch.clone();
    use_effect(move || {
        let Some(fetch) = fetch_for_effect.clone() else {
            return;
        };
        spawn(async move {
            match (fetch)().await {
                Ok(value) => status.set(Some(value)),
                Err(failure) => error.set(Some(failure)),
            }
        });
    });

    match (has_transport, status(), error()) {
        (false, _, _) => rsx! {
            p {
                class: "mt-4 text-slate-400",
                "No API transport was provided by the launcher."
            }
        },
        // A known value beats a later transport failure.
        (true, Some(value), _) => rsx! {
            p { class: "mt-4 text-emerald-400", "Backend: {value.status} ({value.service})" }
        },
        (true, None, Some(failure)) => rsx! {
            p { class: "mt-4 text-amber-400", "Backend unavailable: {failure}" }
        },
        (true, None, None) => rsx! {
            p { class: "mt-4 text-slate-400", "Checking backend..." }
        },
    }
}
