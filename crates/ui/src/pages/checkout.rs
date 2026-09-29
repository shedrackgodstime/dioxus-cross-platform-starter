#[cfg(feature = "fullstack")]
use crate::components::badge::Badge;
#[cfg(feature = "fullstack")]
use crate::components::button::{Button, ButtonVariant};
#[cfg(feature = "fullstack")]
use crate::components::card::Card;
#[cfg(feature = "fullstack")]
use crate::components::icon::{Icon, IconKind};
#[cfg(feature = "fullstack")]
use crate::routes::Route;
use dioxus::prelude::*;
#[cfg(feature = "fullstack")]
use starter_core::{PaymentPlan, PaymentVerificationResult};

#[cfg(feature = "fullstack")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SelectedPlanType {
    Basic,
    Premium,
}

#[cfg(feature = "fullstack")]
#[derive(Clone, PartialEq, Debug)]
pub enum CheckoutStep {
    SelectPlan,
    Initializing,
    AwaitingPayment {
        reference: String,
        access_code: String,
        authorization_url: String,
    },
    VerifyingPayment {
        reference: String,
    },
    Success(PaymentVerificationResult),
    Cancelled,
    Failed {
        message: String,
    },
}

#[component]
pub fn Checkout() -> Element {
    #[cfg(feature = "fullstack")]
    {
        return rsx! { FullstackCheckout {} };
    }

    #[cfg(not(feature = "fullstack"))]
    rsx! {
        div { class: "mx-auto max-w-3xl space-y-4",
            h1 { class: "text-2xl font-bold text-white", "Checkout" }
            p { class: "text-sm text-slate-400", "Paystack checkout is available in the fullstack build. Add your project API adapter before enabling payments in a standalone client." }
        }
    }
}

#[cfg(feature = "fullstack")]
#[component]
fn FullstackCheckout() -> Element {
    let mut selected_plan = use_signal(|| SelectedPlanType::Premium);
    let mut email = use_signal(String::new);
    let mut step = use_signal(|| CheckoutStep::SelectPlan);
    let mut error_text = use_signal(|| Option::<String>::None);

    let plan = match *selected_plan.read() {
        SelectedPlanType::Basic => PaymentPlan::example_basic(),
        SelectedPlanType::Premium => PaymentPlan::example_premium(),
    };

    let amount_naira = plan.amount_kobo() / 100;
    let current_step = step.read().clone();

    rsx! {
        div { class: "max-w-4xl mx-auto space-y-8 animate-fade-in",
            // Page Header
            div { class: "text-center space-y-3",
                div { class: "inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-xs font-semibold tracking-wide uppercase",
                    Icon { kind: IconKind::Shield, class: "w-3.5 h-3.5 text-emerald-400" }
                    "Paystack Inline Integration Example"
                }
                h1 { class: "text-3xl sm:text-4xl font-extrabold text-white tracking-tight",
                    "Example Checkout"
                }
                p { class: "text-slate-400 max-w-xl mx-auto text-sm sm:text-base",
                    "A removable example of server initialization, Paystack Inline checkout, and server-side transaction verification. Replace the example catalog with your application's trusted order logic."
                }
            }

            match current_step {
                CheckoutStep::SelectPlan => rsx! {
                    div { class: "grid grid-cols-1 md:grid-cols-3 gap-6",
                        // Left 2 Columns: Plan selection & Candidate details
                        div { class: "md:col-span-2 space-y-6",
                            // Plan Selection Cards
                            div { class: "space-y-4",
                                h2 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                    Icon { kind: IconKind::Sparkles, class: "w-5 h-5 text-blue-400" }
                                    "1. Choose Your Preparation Plan"
                                }

                                div { class: "grid grid-cols-1 sm:grid-cols-2 gap-4",
                                    // Basic example plan
                                    div {
                                        class: if *selected_plan.read() == SelectedPlanType::Basic {
                                            "relative cursor-pointer rounded-xl border-2 border-blue-500 bg-blue-950/20 p-5 transition-all shadow-md shadow-blue-500/10"
                                        } else {
                                            "relative cursor-pointer rounded-xl border border-slate-700 bg-slate-800/40 p-5 hover:border-slate-600 transition-all"
                                        },
                                        onclick: move |_| *selected_plan.write() = SelectedPlanType::Basic,
                                        div { class: "flex items-start justify-between",
                                            div {
                                                h3 { class: "font-semibold text-slate-100", "Basic Example Plan" }
                                                p { class: "text-xs text-slate-400 mt-1", "Replace with your product" }
                                            }
                                            if *selected_plan.read() == SelectedPlanType::Basic {
                                                Icon { kind: IconKind::CheckCircle, class: "w-5 h-5 text-blue-400" }
                                            }
                                        }
                                        div { class: "mt-4 flex items-baseline gap-1",
                                            span { class: "text-2xl font-black text-white", "₦2,500" }
                                            span { class: "text-xs text-slate-400", "example" }
                                        }
                                        ul { class: "mt-4 space-y-2 text-xs text-slate-300",
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-blue-400" }
                                                "Example entitlement"
                                            }
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-blue-400" }
                                                "Replace with product detail"
                                            }
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-blue-400" }
                                                "Replace with product detail"
                                            }
                                        }
                                    }

                                    // Premium example plan
                                    div {
                                        class: if *selected_plan.read() == SelectedPlanType::Premium {
                                            "relative cursor-pointer rounded-xl border-2 border-emerald-500 bg-emerald-950/20 p-5 transition-all shadow-md shadow-emerald-500/10 ring-1 ring-emerald-500/30"
                                        } else {
                                            "relative cursor-pointer rounded-xl border border-slate-700 bg-slate-800/40 p-5 hover:border-slate-600 transition-all"
                                        },
                                        onclick: move |_| *selected_plan.write() = SelectedPlanType::Premium,
                                        div { class: "absolute -top-3 right-4",
                                            Badge {
                                                label: "Best Value".to_string(),
                                                color_class: "bg-emerald-500 text-slate-950 font-bold border-emerald-400 text-xs shadow-sm".to_string()
                                            }
                                        }
                                        div { class: "flex items-start justify-between",
                                            div {
                                                h3 { class: "font-semibold text-slate-100", "Premium Example Plan" }
                                                p { class: "text-xs text-slate-400 mt-1", "Replace with your product" }
                                            }
                                            if *selected_plan.read() == SelectedPlanType::Premium {
                                                Icon { kind: IconKind::CheckCircle, class: "w-5 h-5 text-emerald-400" }
                                            }
                                        }
                                        div { class: "mt-4 flex items-baseline gap-1",
                                            span { class: "text-2xl font-black text-white", "₦5,000" }
                                            span { class: "text-xs text-slate-400", "/ 1 year" }
                                        }
                                        ul { class: "mt-4 space-y-2 text-xs text-slate-300",
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-emerald-400" }
                                                "Example entitlement"
                                            }
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-emerald-400" }
                                                "Example premium entitlement"
                                            }
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-emerald-400" }
                                                "Replace with product detail"
                                            }
                                            li { class: "flex items-center gap-2",
                                                Icon { kind: IconKind::Check, class: "w-3.5 h-3.5 text-emerald-400" }
                                                "Replace with product detail"
                                            }
                                        }
                                    }
                                }
                            }

                            // Candidate Email Input
                            div { class: "space-y-3 pt-2",
                                h2 { class: "text-lg font-bold text-slate-200 flex items-center gap-2",
                                    Icon { kind: IconKind::Terminal, class: "w-5 h-5 text-blue-400" }
                                    "2. Candidate Email"
                                }
                                div { class: "space-y-1.5",
                                    input {
                                        r#type: "email",
                                        value: "{email}",
                                        placeholder: "you@example.com",
                                        class: "w-full px-4 py-2.5 rounded-lg bg-slate-900 border border-slate-700 text-slate-100 placeholder-slate-500 focus:outline-none focus:border-blue-500 text-sm",
                                        oninput: move |e| *email.write() = e.value(),
                                    }
                                    p { class: "text-xs text-slate-500",
                                        "Use the customer's account email."
                                    }
                                }
                            }
                        }

                        // Right 1 Column: Summary & In-App Checkout Action
                        div { class: "space-y-4",
                            Card {
                                title: Some("Order Summary".to_string()),
                                class: "sticky top-20 border-slate-700/80 bg-slate-900/60".to_string(),
                                div { class: "space-y-4 text-sm",
                                    div { class: "flex justify-between items-center pb-3 border-b border-slate-800",
                                        span { class: "text-slate-400", "Selected Plan" }
                                        span { class: "font-semibold text-slate-200", "{plan.name()}" }
                                    }
                                    div { class: "flex justify-between items-center pb-3 border-b border-slate-800",
                                        span { class: "text-slate-400", "Access Duration" }
                                        span { class: "text-slate-200",
                                            if *selected_plan.read() == SelectedPlanType::Premium {
                                                "12 Months"
                                            } else {
                                                "3 Months"
                                            }
                                        }
                                    }
                                    div { class: "flex justify-between items-center pb-3 border-b border-slate-800",
                                        span { class: "text-slate-400", "Processing Fee" }
                                        span { class: "text-emerald-400 font-medium", "₦0.00 (Free)" }
                                    }
                                    div { class: "flex justify-between items-center pt-1 text-base",
                                        span { class: "font-bold text-slate-200", "Total Due" }
                                        span { class: "font-extrabold text-xl text-white", "₦{amount_naira:?}" }
                                    }

                                    if let Some(err) = error_text.read().as_ref() {
                                        div { class: "p-3 rounded-lg bg-rose-500/10 border border-rose-500/20 text-rose-300 text-xs flex items-start gap-2",
                                            Icon { kind: IconKind::AlertCircle, class: "w-4 h-4 text-rose-400 shrink-0 mt-0.5" }
                                            span { "{err}" }
                                        }
                                    }

                                    // Launch In-App Paystack Modal Button
                                    button {
                                        class: "w-full py-3 px-4 rounded-xl bg-gradient-to-r from-emerald-600 to-teal-600 hover:from-emerald-500 hover:to-teal-500 text-white font-bold text-sm shadow-lg shadow-emerald-900/30 transition-all duration-150 flex items-center justify-center gap-2 active:scale-95 disabled:opacity-50",
                                        onclick: move |_| {
                                            let customer_email = email.read().clone();
                                            let product_id = plan.product_id().to_string();

                                            if !customer_email.contains('@') || !customer_email.contains('.') {
                                                *error_text.write() = Some("Please enter a valid email address.".to_string());
                                                return;
                                            }
                                            *error_text.write() = None;
                                            *step.write() = CheckoutStep::Initializing;

                                            spawn(async move {
                                                // Step 1: Initiate payment via backend server function
                                                match starter_api::initiate_paystack_payment(
                                                    customer_email.clone(),
                                                    product_id,
                                                ).await {
                                                    Ok(init) => {
                                                        let access_code = init.access_code.clone();
                                                        let reference = init.reference.clone();
                                                        let authorization_url = init.authorization_url.clone();

                                                        *step.write() = CheckoutStep::AwaitingPayment {
                                                            reference: reference.clone(),
                                                            access_code: access_code.clone(),
                                                            authorization_url: authorization_url.clone(),
                                                        };

                                                        // Step 2: Open Paystack Inline Modal via Dioxus JS Bridge
                                                        let js_code = format!(
                                                            r#"
                                                            (function() {{
                                                                const accessCode = '{access_code}';
                                                                const ref = '{reference}';
                                                                const authUrl = '{authorization_url}';

                                                                function openPaystack() {{
                                                                    if (typeof PaystackPop === 'undefined') {{
                                                                        const script = document.createElement('script');
                                                                        script.src = 'https://js.paystack.co/v2/inline.js';
                                                                        script.onload = function() {{
                                                                            startPopup();
                                                                        }};
                                                                        script.onerror = function() {{
                                                                            dioxus.send({{
                                                                                status: "fallback",
                                                                                url: authUrl,
                                                                                reference: ref
                                                                            }});
                                                                        }};
                                                                        document.head.appendChild(script);
                                                                    }} else {{
                                                                        startPopup();
                                                                    }}
                                                                }}

                                                                function startPopup() {{
                                                                    try {{
                                                                        const popup = new PaystackPop();
                                                                        popup.resumeTransaction(accessCode, {{
                                                                            onSuccess: function(trx) {{
                                                                                dioxus.send({{
                                                                                    status: "success",
                                                                                    reference: (trx && trx.reference) ? trx.reference : ref
                                                                                }});
                                                                            }},
                                                                            onCancel: function() {{
                                                                                dioxus.send({{ status: "cancelled" }});
                                                                            }},
                                                                            onError: function(err) {{
                                                                                dioxus.send({{
                                                                                    status: "failed",
                                                                                    error: (err && err.message) ? err.message : "Paystack encounter an issue."
                                                                                }});
                                                                            }}
                                                                        }});
                                                                    }} catch (e) {{
                                                                        dioxus.send({{
                                                                            status: "fallback",
                                                                            url: authUrl,
                                                                            reference: ref
                                                                        }});
                                                                    }}
                                                                }}

                                                                openPaystack();
                                                            }})();
                                                            "#
                                                        );

                                                        let mut eval = document::eval(&js_code);

                                                        match eval.recv::<serde_json::Value>().await {
                                                            Ok(msg) => {
                                                                let status = msg.get("status").and_then(|s| s.as_str()).unwrap_or("failed");
                                                                match status {
                                                                    "success" => {
                                                                        let ref_to_verify = msg.get("reference")
                                                                            .and_then(|r| r.as_str())
                                                                            .unwrap_or(&reference)
                                                                            .to_string();

                                                                        *step.write() = CheckoutStep::VerifyingPayment {
                                                                            reference: ref_to_verify.clone(),
                                                                        };

                                                                        // Step 3: Verify reference with backend
                                                                        match starter_api::verify_paystack_payment(ref_to_verify).await {
                                                                            Ok(result) => {
                                                                                if result.is_paid {
                                                                                    *step.write() = CheckoutStep::Success(result);
                                                                                } else {
                                                                                    *step.write() = CheckoutStep::Failed {
                                                                                        message: result.message,
                                                                                    };
                                                                                }
                                                                            }
                                                                            Err(e) => {
                                                                                *step.write() = CheckoutStep::Failed {
                                                                                    message: format!("Verification error: {}", e),
                                                                                };
                                                                            }
                                                                        }
                                                                    }
                                                                    "cancelled" => {
                                                                        *step.write() = CheckoutStep::Cancelled;
                                                                    }
                                                                    "fallback" => {
                                                                        // Keep AwaitingPayment step active so user can click fallback link or manual verify
                                                                    }
                                                                    _ => {
                                                                        let err_detail = msg.get("error")
                                                                            .and_then(|e| e.as_str())
                                                                            .unwrap_or("Payment could not be completed.");
                                                                        *step.write() = CheckoutStep::Failed {
                                                                            message: err_detail.to_string(),
                                                                        };
                                                                    }
                                                                }
                                                            }
                                                            Err(e) => {
                                                                *step.write() = CheckoutStep::Failed {
                                                                    message: format!("IPC communication error: {}", e),
                                                                };
                                                            }
                                                        }
                                                    }
                                                    Err(e) => {
                                                        *step.write() = CheckoutStep::Failed {
                                                            message: format!("Failed to initiate checkout: {}", e),
                                                        };
                                                    }
                                                }
                                            });
                                        },
                                        Icon { kind: IconKind::Lock, class: "w-4 h-4 text-white" }
                                        "Pay ₦{amount_naira:?} with Paystack"
                                    }

                                    // Channels & Security Badges
                                    div { class: "pt-2 text-center space-y-2",
                                        div { class: "flex items-center justify-center gap-3 text-xs text-slate-400",
                                            span { "Debit Card" }
                                            span { "•" }
                                            span { "Bank Transfer" }
                                            span { "•" }
                                            span { "USSD" }
                                            span { "•" }
                                            span { "Apple Pay" }
                                        }
                                        p { class: "text-[11px] text-slate-500 flex items-center justify-center gap-1",
                                            Icon { kind: IconKind::Shield, class: "w-3 h-3 text-slate-500" }
                                            "Secured by Paystack • Zero App Exit"
                                        }
                                    }
                                }
                            }
                        }
                    }
                },

                CheckoutStep::Initializing => rsx! {
                    div { class: "max-w-md mx-auto text-center py-16 space-y-4",
                        div { class: "w-12 h-12 mx-auto rounded-full border-4 border-blue-500/20 border-t-blue-500 animate-spin" }
                        h2 { class: "text-xl font-bold text-white", "Initializing Secure Checkout..." }
                        p { class: "text-sm text-slate-400", "Connecting to Paystack gateway to prepare your payment session." }
                    }
                },

                CheckoutStep::AwaitingPayment { reference, access_code: _, authorization_url } => {
                    let ref_for_verify = reference.clone();
                    rsx! {
                        div { class: "max-w-md mx-auto text-center py-12 space-y-6 bg-slate-900/80 border border-slate-800 rounded-2xl p-8 shadow-xl animate-fade-in",
                            div { class: "relative w-16 h-16 mx-auto flex items-center justify-center",
                                div { class: "absolute inset-0 rounded-full bg-emerald-500/20 animate-ping" }
                                div { class: "relative w-14 h-14 rounded-full bg-emerald-500/30 flex items-center justify-center text-emerald-400",
                                    Icon { kind: IconKind::CreditCard, class: "w-7 h-7" }
                                }
                            }
                            div { class: "space-y-2",
                                h2 { class: "text-xl font-bold text-white", "Paystack Checkout Active" }
                                p { class: "text-sm text-slate-300",
                                    "Please complete your payment in the Paystack modal window."
                                }
                                p { class: "text-xs font-mono text-slate-400 bg-slate-950/60 py-1.5 px-3 rounded-lg border border-slate-800 inline-block",
                                    "Ref: {reference}"
                                }
                            }
                            div { class: "pt-2 flex flex-col gap-3",
                                a {
                                    href: "{authorization_url}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    class: "w-full py-2.5 px-4 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700 font-medium text-xs flex items-center justify-center gap-2 transition",
                                    Icon { kind: IconKind::Globe, class: "w-4 h-4 text-emerald-400" }
                                    "Open Paystack in Browser Tab"
                                }
                                Button {
                                    label: "I Have Paid — Verify Now".to_string(),
                                    variant: ButtonVariant::Primary,
                                    on_click: move |_| {
                                        let ref_clone = ref_for_verify.clone();
                                        *step.write() = CheckoutStep::VerifyingPayment { reference: ref_clone.clone() };
                                        spawn(async move {
                                            match starter_api::verify_paystack_payment(ref_clone).await {
                                                Ok(result) => {
                                                    if result.is_paid {
                                                        *step.write() = CheckoutStep::Success(result);
                                                    } else {
                                                        *step.write() = CheckoutStep::Failed {
                                                            message: format!("Payment not completed yet: {}", result.message),
                                                        };
                                                    }
                                                }
                                                Err(e) => {
                                                    *step.write() = CheckoutStep::Failed {
                                                        message: format!("Verification error: {}", e),
                                                    };
                                                }
                                            }
                                        });
                                    },
                                }
                                Button {
                                    label: "Cancel Transaction".to_string(),
                                    variant: ButtonVariant::Outline,
                                    on_click: move |_| *step.write() = CheckoutStep::Cancelled,
                                }
                            }
                        }
                    }
                },

                CheckoutStep::VerifyingPayment { reference } => rsx! {
                    div { class: "max-w-md mx-auto text-center py-16 space-y-4",
                        div { class: "w-12 h-12 mx-auto rounded-full border-4 border-emerald-500/20 border-t-emerald-500 animate-spin" }
                        h2 { class: "text-xl font-bold text-white", "Verifying Payment..." }
                        p { class: "text-sm text-slate-400", "Confirming transaction reference with Paystack servers." }
                        p { class: "text-xs font-mono text-slate-500", "{reference}" }
                    }
                },

                CheckoutStep::Success(verified) => rsx! {
                    div { class: "max-w-lg mx-auto bg-slate-900/80 border border-emerald-500/30 rounded-2xl p-8 shadow-2xl text-center space-y-6 animate-fade-in",
                        div { class: "w-16 h-16 mx-auto rounded-full bg-emerald-500/20 border border-emerald-500/30 flex items-center justify-center text-emerald-400 shadow-lg shadow-emerald-500/10",
                            Icon { kind: IconKind::CheckCircle, class: "w-8 h-8" }
                        }
                        div { class: "space-y-2",
                            h2 { class: "text-2xl font-black text-white", "Payment Successful!" }
                            p { class: "text-slate-300 text-sm",
                                "Payment verified. Connect this result to your application's order fulfillment logic."
                            }
                        }

                        // Receipt Details Table
                        div { class: "bg-slate-950/60 rounded-xl p-4 border border-slate-800 text-xs space-y-2.5 text-left font-sans",
                            div { class: "flex justify-between",
                                span { class: "text-slate-400", "Reference:" }
                                span { class: "font-mono font-bold text-slate-200", "{verified.reference}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-slate-400", "Amount Paid:" }
                                span { class: "font-bold text-emerald-400", "₦{(verified.amount_kobo / 100):?}" }
                            }
                            div { class: "flex justify-between",
                                span { class: "text-slate-400", "Payment Channel:" }
                                span { class: "capitalize text-slate-200", "{verified.channel}" }
                            }
                            if let Some(paid_at) = &verified.paid_at {
                                div { class: "flex justify-between",
                                    span { class: "text-slate-400", "Time:" }
                                    span { class: "text-slate-300", "{paid_at}" }
                                }
                            }
                        }

                        div { class: "pt-2 flex flex-col sm:flex-row gap-3 justify-center",
                            Link {
                                to: Route::Home {},
                                class: "px-6 py-2.5 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white font-bold text-sm shadow-md transition-all flex items-center justify-center gap-2",
                                Icon { kind: IconKind::Sparkles, class: "w-4 h-4" }
                                "Start Practicing Now"
                            }
                            Button {
                                label: "Purchase Another Plan".to_string(),
                                variant: ButtonVariant::Outline,
                                on_click: move |_| *step.write() = CheckoutStep::SelectPlan,
                            }
                        }
                    }
                },

                CheckoutStep::Cancelled => rsx! {
                    div { class: "max-w-md mx-auto text-center py-12 space-y-5 bg-slate-900/60 border border-slate-800 rounded-2xl p-8",
                        div { class: "w-14 h-14 mx-auto rounded-full bg-amber-500/20 text-amber-400 flex items-center justify-center",
                            Icon { kind: IconKind::AlertCircle, class: "w-7 h-7" }
                        }
                        div { class: "space-y-2",
                            h2 { class: "text-xl font-bold text-white", "Transaction Dismissed" }
                            p { class: "text-sm text-slate-400",
                                "The checkout window was closed before completing payment. Your account was not charged."
                            }
                        }
                        div { class: "pt-2",
                            Button {
                                label: "Return to Checkout".to_string(),
                                variant: ButtonVariant::Primary,
                                on_click: move |_| *step.write() = CheckoutStep::SelectPlan,
                            }
                        }
                    }
                },

                CheckoutStep::Failed { message } => rsx! {
                    div { class: "max-w-md mx-auto text-center py-12 space-y-5 bg-slate-900/60 border border-rose-500/30 rounded-2xl p-8 shadow-xl",
                        div { class: "w-14 h-14 mx-auto rounded-full bg-rose-500/20 text-rose-400 flex items-center justify-center",
                            Icon { kind: IconKind::AlertCircle, class: "w-7 h-7" }
                        }
                        div { class: "space-y-2",
                            h2 { class: "text-xl font-bold text-white", "Payment Failed" }
                            p { class: "text-sm text-rose-300", "{message}" }
                        }
                        div { class: "pt-2 flex gap-3 justify-center",
                            Button {
                                label: "Try Again".to_string(),
                                variant: ButtonVariant::Primary,
                                on_click: move |_| *step.write() = CheckoutStep::SelectPlan,
                            }
                            Link {
                                to: Route::Home {},
                                class: "px-4 py-2 rounded-lg border border-slate-600 text-slate-300 text-sm hover:text-white transition",
                                "Back to Home"
                            }
                        }
                    }
                },
            }
        }
    }
}
