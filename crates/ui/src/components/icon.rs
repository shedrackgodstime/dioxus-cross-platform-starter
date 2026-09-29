use dioxus::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IconKind {
    Home,
    Layers,
    Cpu,
    Settings,
    Terminal,
    Check,
    Sparkles,
    Refresh,
    Mobile,
    Desktop,
    Globe,
    Server,
    CreditCard,
    Shield,
    CheckCircle,
    AlertCircle,
    Lock,
}

#[derive(Props, Clone, PartialEq)]
pub struct IconProps {
    pub kind: IconKind,
    #[props(default = "w-5 h-5".to_string())]
    pub class: String,
}

#[component]
pub fn Icon(props: IconProps) -> Element {
    let base_svg = match props.kind {
        IconKind::Home => rsx! {
            path { d: "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
            polyline { points: "9 22 9 12 15 12 15 22" }
        },
        IconKind::Layers => rsx! {
            polygon { points: "12 2 2 7 12 12 22 7 12 2" }
            polyline { points: "2 17 12 22 22 17" }
            polyline { points: "2 12 12 17 22 12" }
        },
        IconKind::Cpu => rsx! {
            rect { x: "4", y: "4", width: "16", height: "16", rx: "2" }
            rect { x: "9", y: "9", width: "6", height: "6" }
            line { x1: "9", y1: "1", x2: "9", y2: "4" }
            line { x1: "15", y1: "1", x2: "15", y2: "4" }
            line { x1: "9", y1: "20", x2: "9", y2: "23" }
            line { x1: "15", y1: "20", x2: "15", y2: "23" }
            line { x1: "20", y1: "9", x2: "23", y2: "9" }
            line { x1: "20", y1: "14", x2: "23", y2: "14" }
            line { x1: "1", y1: "9", x2: "4", y2: "9" }
            line { x1: "1", y1: "14", x2: "4", y2: "14" }
        },
        IconKind::Settings => rsx! {
            circle { cx: "12", cy: "12", r: "3" }
            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z" }
        },
        IconKind::Terminal => rsx! {
            polyline { points: "4 17 10 11 4 5" }
            line { x1: "12", y1: "19", x2: "20", y2: "19" }
        },
        IconKind::Check => rsx! {
            polyline { points: "20 6 9 17 4 12" }
        },
        IconKind::Sparkles => rsx! {
            path { d: "M12 2l2.4 7.2L22 12l-7.6 2.8L12 22l-2.4-7.2L2 12l7.6-2.8z" }
        },
        IconKind::Refresh => rsx! {
            polyline { points: "23 4 23 10 17 10" }
            polyline { points: "1 20 1 14 7 14" }
            path { d: "M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" }
        },
        IconKind::Mobile => rsx! {
            rect { x: "5", y: "2", width: "14", height: "20", rx: "2", ry: "2" }
            line { x1: "12", y1: "18", x2: "12.01", y2: "18" }
        },
        IconKind::Desktop => rsx! {
            rect { x: "2", y: "3", width: "20", height: "14", rx: "2", ry: "2" }
            line { x1: "8", y1: "21", x2: "16", y2: "21" }
            line { x1: "12", y1: "17", x2: "12", y2: "21" }
        },
        IconKind::Globe => rsx! {
            circle { cx: "12", cy: "12", r: "10" }
            line { x1: "2", y1: "12", x2: "22", y2: "12" }
            path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
        },
        IconKind::Server => rsx! {
            rect { x: "2", y: "2", width: "20", height: "8", rx: "2", ry: "2" }
            rect { x: "2", y: "14", width: "20", height: "8", rx: "2", ry: "2" }
            line { x1: "6", y1: "6", x2: "6.01", y2: "6" }
            line { x1: "6", y1: "18", x2: "6.01", y2: "18" }
        },
        IconKind::CreditCard => rsx! {
            rect { x: "1", y: "4", width: "22", height: "16", rx: "2", ry: "2" }
            line { x1: "1", y1: "10", x2: "23", y2: "10" }
        },
        IconKind::Shield => rsx! {
            path { d: "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" }
        },
        IconKind::CheckCircle => rsx! {
            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
            polyline { points: "22 4 12 14.01 9 11.01" }
        },
        IconKind::AlertCircle => rsx! {
            circle { cx: "12", cy: "12", r: "10" }
            line { x1: "12", y1: "8", x2: "12", y2: "12" }
            line { x1: "12", y1: "16", x2: "12.01", y2: "16" }
        },
        IconKind::Lock => rsx! {
            rect { x: "3", y: "11", width: "18", height: "11", rx: "2", ry: "2" }
            path { d: "M7 11V7a5 5 0 0 1 10 0v4" }
        },
    };

    rsx! {
        svg {
            class: "{props.class}",
            width: "20",
            height: "20",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            {base_svg}
        }
    }
}
