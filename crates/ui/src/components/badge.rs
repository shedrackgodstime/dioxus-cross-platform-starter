use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    pub label: String,
    #[props(default = "bg-blue-500/20 text-blue-400 border-blue-500/30".to_string())]
    pub color_class: String,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium border {props.color_class}",
            "{props.label}"
        }
    }
}
