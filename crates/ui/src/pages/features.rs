use crate::components::{Button, ButtonVariant, Card};
use crate::state::PlatformContext;
use dioxus::prelude::*;

#[component]
pub fn Features() -> Element {
    let platform = use_context::<PlatformContext>();

    let mut key_input = use_signal(|| "demo_key".to_string());
    let mut value_input = use_signal(|| "Hello from storage!".to_string());
    let mut stored_value = use_signal(|| None::<String>);
    let mut status_msg = use_signal(|| String::new());

    let on_save = {
        let storage = platform.storage.clone();
        move |_| {
            let key = (key_input)();
            let val = (value_input)();
            match storage.set(&key, &val) {
                Ok(_) => status_msg.set(format!("Saved '{key}' successfully")),
                Err(e) => status_msg.set(format!("Save error: {e}")),
            }
        }
    };

    let on_load = {
        let storage = platform.storage.clone();
        move |_| {
            let key = (key_input)();
            match storage.get(&key) {
                Ok(val) => {
                    stored_value.set(val.clone());
                    status_msg.set(if val.is_some() {
                        format!("Loaded '{key}'")
                    } else {
                        format!("Key '{key}' not found")
                    });
                }
                Err(e) => status_msg.set(format!("Load error: {e}")),
            }
        }
    };

    rsx! {
        div { class: "space-y-6 max-w-4xl mx-auto",
            div { class: "space-y-1",
                h2 { class: "text-2xl font-bold text-white", "Cross-Platform Storage Port" }
                p { class: "text-sm text-slate-400",
                    "Hexagonal architecture: UI calls the abstract KeyValueStore port. "
                    "Web uses browser localStorage, Desktop uses JSON file storage, and Android uses app-private filesDir via JNI."
                }
            }

            Card { title: "Test Platform Persistence".to_string(),
                div { class: "space-y-4",
                    div { class: "grid sm:grid-cols-2 gap-4",
                        div { class: "space-y-1",
                            label { class: "text-xs font-medium text-slate-300", "Key" }
                            input {
                                class: "w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500",
                                value: "{key_input}",
                                oninput: move |e| key_input.set(e.value()),
                            }
                        }
                        div { class: "space-y-1",
                            label { class: "text-xs font-medium text-slate-300", "Value" }
                            input {
                                class: "w-full px-3 py-2 bg-slate-900 border border-slate-700 rounded-lg text-sm text-slate-100 focus:outline-none focus:border-blue-500",
                                value: "{value_input}",
                                oninput: move |e| value_input.set(e.value()),
                            }
                        }
                    }

                    div { class: "flex gap-3",
                        Button {
                            label: "Save to Local Store".to_string(),
                            variant: ButtonVariant::Primary,
                            on_click: on_save,
                        }
                        Button {
                            label: "Load from Store".to_string(),
                            variant: ButtonVariant::Secondary,
                            on_click: on_load,
                        }
                    }

                    if !(status_msg)().is_empty() {
                        div { class: "p-3 bg-slate-900/80 rounded-lg text-xs font-mono text-blue-400 border border-slate-700",
                            "{(status_msg)()}"
                            if let Some(val) = (stored_value)() {
                                div { class: "mt-1 text-emerald-400", "Stored Content: \"{val}\"" }
                            }
                        }
                    }
                }
            }
        }
    }
}
