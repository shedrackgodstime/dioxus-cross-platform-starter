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
        }
    }
}
