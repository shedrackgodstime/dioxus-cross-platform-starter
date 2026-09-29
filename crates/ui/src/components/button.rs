use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Danger,
    Outline,
}

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub on_click: EventHandler<MouseEvent>,
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default = false)]
    pub disabled: bool,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let base_classes = "px-4 py-2 rounded-lg font-medium text-sm transition-all duration-150 active:scale-95 disabled:opacity-50 disabled:pointer-events-none";
    let variant_classes = match props.variant {
        ButtonVariant::Primary => {
            "bg-blue-600 hover:bg-blue-500 text-white shadow-sm shadow-blue-500/20"
        }
        ButtonVariant::Secondary => "bg-slate-700 hover:bg-slate-600 text-slate-100",
        ButtonVariant::Danger => "bg-rose-600 hover:bg-rose-500 text-white",
        ButtonVariant::Outline => {
            "border border-slate-600 hover:border-slate-500 text-slate-200 bg-transparent"
        }
    };

    rsx! {
        button {
            class: "{base_classes} {variant_classes}",
            disabled: props.disabled,
            onclick: move |e| props.on_click.call(e),
            "{props.label}"
        }
    }
}
