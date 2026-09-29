use crate::components::Card;
#[cfg(any(feature = "fullstack", feature = "web-client"))]
use crate::state::PlatformContext;
use dioxus::prelude::*;
#[cfg(any(feature = "fullstack", feature = "web-client"))]
use starter_core::ServerStatus;

#[component]
pub fn SystemInfo() -> Element {
    #[cfg(any(feature = "fullstack", feature = "web-client"))]
    {
        return rsx! { FullstackSystemInfo {} };
    }

    #[cfg(not(any(feature = "fullstack", feature = "web-client")))]
    rsx! {
        div { class: "space-y-6 max-w-4xl mx-auto",
            h2 { class: "text-2xl font-bold text-white", "System & Runtime Diagnostics" }
            Card { title: "Backend Server Telemetry".to_string(),
                p { class: "text-sm text-slate-400", "This standalone web client has no embedded server functions. Configure an API client for your project when you need backend access." }
            }
        }
    }
}

#[cfg(any(feature = "fullstack", feature = "web-client"))]
#[component]
fn FullstackSystemInfo() -> Element {
    let platform = use_context::<PlatformContext>();
    let mut server_status = use_signal(|| None::<ServerStatus>);
    let mut status_error = use_signal(|| None::<String>);

    use_effect(move || {
        spawn(async move {
            match starter_api::get_server_status().await {
                Ok(status) => server_status.set(Some(status)),
                Err(e) => status_error.set(Some(format!("{e:?}"))),
            }
        });
    });

    rsx! {
        div { class: "space-y-6 max-w-4xl mx-auto",
            div { class: "space-y-1",
                h2 { class: "text-2xl font-bold text-white", "System & Runtime Diagnostics" }
                p { class: "text-sm text-slate-400",
                    "Detailed execution context, runtime platform detection, and remote backend telemetry."
                }
            }

            div { class: "grid sm:grid-cols-2 gap-6",
                Card { title: "Client Platform".to_string(),
                    dl { class: "divide-y divide-slate-700/60 text-sm",
                        div { class: "py-2.5 flex justify-between",
                            dt { class: "text-slate-400", "Runtime Target" }
                            dd { class: "font-mono font-medium text-blue-400", "{platform.target.name()}" }
                        }
                        div { class: "py-2.5 flex justify-between",
                            dt { class: "text-slate-400", "Is Mobile" }
                            dd { class: "font-mono text-slate-200", "{platform.target.is_mobile()}" }
                        }
                        div { class: "py-2.5 flex justify-between",
                            dt { class: "text-slate-400", "Is Desktop" }
                            dd { class: "font-mono text-slate-200", "{platform.target.is_desktop()}" }
                        }
                        div { class: "py-2.5 flex justify-between",
                            dt { class: "text-slate-400", "Server Base URL" }
                            dd { class: "font-mono text-slate-300 text-xs truncate max-w-[200px]", "{platform.server_url}" }
                        }
                    }
                }

                Card { title: "Backend Server Telemetry".to_string(),
                    if let Some(status) = (server_status)() {
                        dl { class: "divide-y divide-slate-700/60 text-sm",
                            div { class: "py-2.5 flex justify-between",
                                dt { class: "text-slate-400", "Status" }
                                dd { class: "font-medium text-emerald-400", if status.ok { "Operational" } else { "Degraded" } }
                            }
                            div { class: "py-2.5 flex justify-between",
                                dt { class: "text-slate-400", "Environment" }
                                dd { class: "font-mono text-slate-200", "{status.environment}" }
                            }
                            div { class: "py-2.5 flex justify-between",
                                dt { class: "text-slate-400", "Uptime" }
                                dd { class: "font-mono text-slate-200", "{status.uptime_seconds}s" }
                            }
                            div { class: "py-2.5 flex justify-between",
                                dt { class: "text-slate-400", "Message" }
                                dd { class: "text-xs text-slate-300", "{status.message}" }
                            }
                        }
                    } else if let Some(err) = (status_error)() {
                        div { class: "text-xs text-rose-400 bg-rose-950/30 p-3 rounded-lg border border-rose-800/40",
                            "Failed to reach backend server: {err}"
                        }
                    } else {
                        div { class: "text-xs text-slate-400 py-4 text-center",
                            "Probing server health..."
                        }
                    }
                }
            }
        }
    }
}
