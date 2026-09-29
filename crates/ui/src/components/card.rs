use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct CardProps {
    pub title: Option<String>,
    pub children: Element,
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn Card(props: CardProps) -> Element {
    rsx! {
        div {
            class: "bg-slate-800/80 border border-slate-700/80 rounded-xl p-5 shadow-lg backdrop-blur-sm {props.class}",
            if let Some(title) = &props.title {
                h3 { class: "text-lg font-semibold text-slate-100 mb-3", "{title}" }
            }
            {props.children}
        }
    }
}
