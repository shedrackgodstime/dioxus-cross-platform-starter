use dioxus::prelude::*;

static TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

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
    #[cfg(feature = "fullstack")]
    {
        let status = use_server_future(api::get_server_status)?;

        return match status() {
            Some(Ok(status)) => rsx! {
                p {
                    class: "mt-4 text-emerald-400",
                    "Backend: {status.status} ({status.service})"
                }
            },
            Some(Err(error)) => rsx! {
                p {
                    class: "mt-4 text-amber-400",
                    "Backend unavailable: {error}"
                }
            },
            None => rsx! {
                p {
                    class: "mt-4 text-slate-400",
                    "Checking backend..."
                }
            },
        };
    }

    #[cfg(not(feature = "fullstack"))]
    rsx! {}
}
