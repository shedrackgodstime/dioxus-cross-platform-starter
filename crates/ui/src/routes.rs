use crate::components::Navbar;
use crate::pages::{Checkout, Home, Settings, SystemInfo};
use dioxus::prelude::*;

pub const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
pub const LOGO_SVG: Asset = asset!("/assets/logo.svg");

#[component]
fn AppLayout() -> Element {
    use_effect(move || {
        let _ = document::eval("window.__dismissSplash && window.__dismissSplash();");
    });

    rsx! {
        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no, viewport-fit=cover" }
        document::Meta { name: "theme-color", content: "#0f172a" }
        document::Meta { name: "color-scheme", content: "light dark" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "icon", href: LOGO_SVG }
        document::Style {
            "html, body {{ background-color: #020617 !important; color: #f8fafc; margin: 0; padding: 0; }} html[data-theme='light'] body {{ background-color: #f8fafc !important; color: #0f172a; }} html[data-theme='light'] .app-shell {{ background-color: #f8fafc !important; color: #0f172a; }} svg {{ width: 1.25rem; height: 1.25rem; }}"
        }

        div { class: "app-shell min-h-screen bg-slate-950 text-slate-100 flex flex-col font-sans selection:bg-blue-500/30",
            Navbar {}
            main { class: "flex-1 max-w-6xl w-full mx-auto p-4 sm:p-6 lg:p-8",
                Outlet::<Route> {}
            }
            footer { class: "border-t border-slate-800/80 py-4 text-center text-xs text-slate-500",
                "Universal Dioxus 0.7 Starter • Web, Desktop, Mobile & Server"
            }
        }
    }
}

/// Unified Route enum.
/// CRITICAL: Zero `#[cfg]` gates on enum variants.
/// All routes are compiled on all platforms to guarantee exhaustive matching.
#[derive(Routable, Clone, PartialEq, Debug)]
pub enum Route {
    #[layout(AppLayout)]
    #[route("/")]
    Home {},

    #[route("/system")]
    SystemInfo {},

    #[route("/settings")]
    Settings {},

    #[route("/checkout")]
    Checkout {},
}
