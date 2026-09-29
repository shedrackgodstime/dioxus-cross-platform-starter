use crate::components::badge::Badge;
use crate::components::icon::{Icon, IconKind};
use crate::routes::{LOGO_SVG, Route};
use crate::state::PlatformContext;
use dioxus::prelude::*;

#[component]
pub fn Navbar() -> Element {
    let platform = use_context::<PlatformContext>();

    rsx! {
        nav { class: "border-b border-slate-800 bg-slate-900/90 sticky top-0 z-50 backdrop-blur px-4 py-3",
            div { class: "max-w-6xl mx-auto flex items-center justify-between",
                div { class: "flex items-center gap-3",
                    Link {
                        to: Route::Home {},
                        class: "text-lg font-bold text-transparent bg-clip-text bg-gradient-to-r from-blue-400 to-indigo-400 flex items-center gap-2",
                        img {
                            src: LOGO_SVG,
                            alt: "App Logo",
                            class: "w-7 h-7 rounded-lg shadow-sm shadow-blue-500/30",
                        }
                        span { "Dioxus Starter" }
                    }
                    Badge {
                        label: platform.target.name().to_string(),
                        color_class: "bg-indigo-500/20 text-indigo-300 border-indigo-500/30".to_string()
                    }
                }
                div { class: "flex items-center gap-4 text-sm font-medium",
                    Link {
                        to: Route::Home {},
                        class: "text-slate-300 hover:text-white transition flex items-center gap-1.5",
                        Icon { kind: IconKind::Home, class: "w-4 h-4 text-slate-400" }
                        span { class: "hidden sm:inline", "Home" }
                    }
                    Link {
                        to: Route::SystemInfo {},
                        class: "text-slate-300 hover:text-white transition flex items-center gap-1.5",
                        Icon { kind: IconKind::Cpu, class: "w-4 h-4 text-slate-400" }
                        span { class: "hidden sm:inline", "System" }
                    }
                    Link {
                        to: Route::Settings {},
                        class: "text-slate-300 hover:text-white transition flex items-center gap-1.5",
                        Icon { kind: IconKind::Settings, class: "w-4 h-4 text-slate-400" }
                        span { class: "hidden sm:inline", "Settings" }
                    }
                    Link {
                        to: Route::Checkout {},
                        class: "px-2.5 py-1 rounded-md bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-medium text-xs shadow-sm shadow-emerald-900/30 transition flex items-center gap-1.5",
                        Icon { kind: IconKind::CreditCard, class: "w-3.5 h-3.5 text-white" }
                        span { "Paystack" }
                    }
                }
            }
        }
    }
}
