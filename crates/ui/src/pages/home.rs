use crate::components::Card;
use crate::routes::Route;
use crate::state::PlatformContext;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let platform = use_context::<PlatformContext>();

    rsx! {
        div { class: "mx-auto max-w-4xl space-y-8 py-8",
            header { class: "space-y-3",
                p { class: "text-sm font-semibold uppercase tracking-widest text-blue-400",
                    "Personal project foundation"
                }
                h1 { class: "text-3xl font-bold tracking-tight text-white sm:text-4xl",
                    "Start with the platform plumbing in place."
                }
                p { class: "max-w-2xl text-slate-300",
                    "Shared Dioxus UI, platform launchers, a typed fullstack boundary, backend services, persistent storage adapters, and an optional Paystack Inline integration live in this workspace."
                }
            }

            div { class: "grid gap-5 md:grid-cols-2",
                Card { title: "Current runtime".to_string(),
                    dl { class: "space-y-3 text-sm",
                        div { class: "flex justify-between gap-4",
                            dt { class: "text-slate-400", "Platform" }
                            dd { class: "font-medium text-slate-100", "{platform.target.name()}" }
                        }
                        div { class: "flex justify-between gap-4",
                            dt { class: "text-slate-400", "Server functions" }
                            dd { class: "font-medium text-slate-100", "{platform.server_url}" }
                        }
                    }
                    Link { to: Route::SystemInfo {}, class: "mt-4 inline-block text-sm text-blue-400 hover:text-blue-300",
                        "Open runtime diagnostics →"
                    }
                }

                Card { title: "Reusable integration".to_string(),
                    p { class: "text-sm text-slate-300",
                        "The Paystack example demonstrates server-side initialization, Inline checkout handoff, and server-side verification. Connect it to trusted project order logic before enabling real fulfillment."
                    }
                    Link { to: Route::Checkout {}, class: "mt-4 inline-block text-sm text-emerald-400 hover:text-emerald-300",
                        "Open Paystack example →"
                    }
                }
            }
        }
    }
}
