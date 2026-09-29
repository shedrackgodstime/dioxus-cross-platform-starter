use serde::{Deserialize, Serialize};

/// Target platform identifier detected at runtime or launcher boot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformTarget {
    Web,
    Desktop(DesktopOs),
    Mobile(MobileOs),
    Server,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesktopOs {
    MacOS,
    Windows,
    Linux,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileOs {
    Android,
    Ios,
    Unknown,
}

impl PlatformTarget {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Web => "Web (WASM)",
            Self::Desktop(DesktopOs::MacOS) => "Desktop (macOS)",
            Self::Desktop(DesktopOs::Windows) => "Desktop (Windows)",
            Self::Desktop(DesktopOs::Linux) => "Desktop (Linux)",
            Self::Desktop(DesktopOs::Unknown) => "Desktop (Native)",
            Self::Mobile(MobileOs::Android) => "Mobile (Android)",
            Self::Mobile(MobileOs::Ios) => "Mobile (iOS)",
            Self::Mobile(MobileOs::Unknown) => "Mobile (Native)",
            Self::Server => "Server (SSR/API)",
        }
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::Mobile(_))
    }

    pub fn is_desktop(&self) -> bool {
        matches!(self, Self::Desktop(_))
    }
}

/// Server health and status payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerStatus {
    pub ok: bool,
    pub uptime_seconds: u64,
    pub message: String,
    pub environment: String,
}

/// Subscription and purchase plans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PaymentPlan {
    Basic { amount_kobo: u64, name: String },
    Premium { amount_kobo: u64, name: String },
}

impl PaymentPlan {
    pub fn example_basic() -> Self {
        Self::Basic {
            amount_kobo: 250_000, // ₦2,500
            name: "Example Basic Plan".to_string(),
        }
    }

    pub fn example_premium() -> Self {
        Self::Premium {
            amount_kobo: 500_000, // ₦5,000
            name: "Example Premium Plan".to_string(),
        }
    }

    pub fn amount_kobo(&self) -> u64 {
        match self {
            Self::Basic { amount_kobo, .. } | Self::Premium { amount_kobo, .. } => *amount_kobo,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Basic { name, .. } | Self::Premium { name, .. } => name,
        }
    }

    pub fn product_id(&self) -> &'static str {
        match self {
            Self::Basic { .. } => "basic",
            Self::Premium { .. } => "premium",
        }
    }

    pub fn from_product_id(product_id: &str) -> Option<Self> {
        match product_id {
            "basic" => Some(Self::example_basic()),
            "premium" => Some(Self::example_premium()),
            _ => None,
        }
    }
}

/// Result returned from server upon initializing a Paystack transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaystackInitResult {
    pub access_code: String,
    pub reference: String,
    pub amount_kobo: u64,
    pub authorization_url: String,
}

/// Detailed verification result returned after payment processing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaymentVerificationResult {
    pub reference: String,
    pub is_paid: bool,
    pub amount_kobo: u64,
    pub channel: String,
    pub message: String,
    pub paid_at: Option<String>,
}

/// Client-side bridge event status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientPaymentStatus {
    Success { reference: String },
    Cancelled,
    Failed { error: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payment_plan_defaults() {
        let practice = PaymentPlan::example_basic();
        assert_eq!(practice.amount_kobo(), 250_000);
        assert_eq!(practice.name(), "Example Basic Plan");

        let unlimited = PaymentPlan::example_premium();
        assert_eq!(unlimited.amount_kobo(), 500_000);
        assert_eq!(unlimited.name(), "Example Premium Plan");
        assert_eq!(PaymentPlan::from_product_id("basic"), Some(practice));
        assert_eq!(PaymentPlan::from_product_id("premium"), Some(unlimited));
        assert_eq!(PaymentPlan::from_product_id("arbitrary-price"), None);
    }

    #[test]
    fn test_payment_model_serde() {
        let init = PaystackInitResult {
            access_code: "acc_123".to_string(),
            reference: "ref_123".to_string(),
            amount_kobo: 250_000,
            authorization_url: "https://checkout.paystack.com/acc_123".to_string(),
        };
        let json = serde_json::to_string(&init).unwrap();
        let deserialized: PaystackInitResult = serde_json::from_str(&json).unwrap();
        assert_eq!(init, deserialized);

        let verify = PaymentVerificationResult {
            reference: "ref_123".to_string(),
            is_paid: true,
            amount_kobo: 250_000,
            channel: "card".to_string(),
            message: "Successful".to_string(),
            paid_at: Some("2026-09-29T10:00:00Z".to_string()),
        };
        let v_json = serde_json::to_string(&verify).unwrap();
        let v_deserialized: PaymentVerificationResult = serde_json::from_str(&v_json).unwrap();
        assert_eq!(verify, v_deserialized);
    }
}
