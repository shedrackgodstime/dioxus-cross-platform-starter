use crate::components::Card;
use crate::state::PlatformContext;
use dioxus::prelude::*;

#[component]
pub fn Settings() -> Element {
    let platform = use_context::<PlatformContext>();

    rsx! {
        div { class: "mx-auto max-w-3xl space-y-6",
            header { class: "space-y-1",
                h1 { class: "text-2xl font-bold text-white", "Project settings" }
                p { class: "text-sm text-slate-400",
                    "Add product-specific preferences here. The starter keeps platform and server configuration visible for diagnosis."
                }
            }

            Card { title: "Runtime configuration".to_string(),
                dl { class: "space-y-3 text-sm",
                    div { class: "flex flex-col gap-1 sm:flex-row sm:justify-between",
                        dt { class: "text-slate-400", "Platform" }
                        dd { class: "text-slate-100", "{platform.target.name()}" }
                    }
                    div { class: "flex flex-col gap-1 sm:flex-row sm:justify-between",
                        dt { class: "text-slate-400", "Server functions URL" }
                        dd { class: "break-all font-mono text-xs text-emerald-300", "{platform.server_url}" }
                    }
                }
            }

            Card { title: "Startup experience".to_string(),
                div { class: "flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between",
                    p { class: "text-sm text-slate-300", "Preview the shared startup splash screen." }
                    button {
                        class: "rounded-lg border border-indigo-500 bg-indigo-600/80 px-3 py-2 text-xs font-medium text-white hover:bg-indigo-600",
                        onclick: move |_| {
                            let _ = document::eval("window.__showSplashScreen && window.__showSplashScreen(1800);");
                        },
                        "Replay splash"
                    }
                }
            }
        }
    }
}
