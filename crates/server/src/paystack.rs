use ring::hmac;
use serde::{Deserialize, Serialize};
use starter_core::{PaymentVerificationResult, PaystackInitResult};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

static SIM_TX_COUNTER: AtomicU64 = AtomicU64::new(1001);
static SIM_TRANSACTIONS: OnceLock<Mutex<std::collections::HashMap<String, u64>>> = OnceLock::new();

fn paystack_user_agent() -> String {
    std::env::var("PAYSTACK_USER_AGENT")
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| format!("CrossPlatformStarter/{}", env!("CARGO_PKG_VERSION")))
}

/// Errors originating from the Paystack integration.
#[derive(Debug, thiserror::Error)]
pub enum PaystackError {
    #[error("HTTP transport error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Paystack API error: {0}")]
    Api(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Invalid webhook signature")]
    InvalidSignature,
}

/// Raw Paystack initialization request body.
#[derive(Debug, Serialize)]
struct InitializePayload<'a> {
    email: &'a str,
    amount: u64,
    currency: &'a str,
    metadata: serde_json::Value,
}

/// Raw Paystack initialization response.
#[derive(Debug, Deserialize)]
struct PaystackInitResponse {
    status: bool,
    message: String,
    data: Option<PaystackInitData>,
}

#[derive(Debug, Deserialize)]
struct PaystackInitData {
    #[serde(default)]
    authorization_url: Option<String>,
    access_code: String,
    reference: String,
}

/// Raw Paystack verification response.
#[derive(Debug, Deserialize)]
struct PaystackVerifyResponse {
    status: bool,
    message: String,
    data: Option<PaystackVerifyData>,
}

#[derive(Debug, Deserialize)]
struct PaystackVerifyData {
    status: String,
    reference: String,
    amount: u64,
    #[serde(default)]
    channel: Option<String>,
    #[serde(default)]
    gateway_response: Option<String>,
    #[serde(default)]
    paid_at: Option<String>,
}

/// Paystack client providing typed payment operations and webhook signature verification.
#[derive(Clone, Debug)]
pub struct PaystackClient {
    secret_key: Option<String>,
    simulation: bool,
    http: reqwest::Client,
}

impl Default for PaystackClient {
    fn default() -> Self {
        Self::from_env()
    }
}

impl PaystackClient {
    /// Create a new Paystack client with the specified secret key.
    pub fn new(secret_key: impl Into<String>) -> Self {
        let key = secret_key.into().trim().to_string();
        Self {
            simulation: cfg!(debug_assertions)
                && (key == "test_simulation" || key.starts_with("sim_") || key == "mock"),
            secret_key: if key.is_empty() { None } else { Some(key) },
            http: reqwest::Client::new(),
        }
    }

    /// Instantiate client by inspecting `PAYSTACK_SECRET_KEY` in environment variables.
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv();
        let secret_key = std::env::var("PAYSTACK_SECRET_KEY")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());

        let simulation = cfg!(debug_assertions)
            && std::env::var("PAYSTACK_MODE")
                .is_ok_and(|mode| mode.eq_ignore_ascii_case("simulation"));

        Self {
            secret_key,
            simulation,
            http: reqwest::Client::new(),
        }
    }

    /// Check if the client is running in local simulation fallback mode.
    pub fn is_simulation(&self) -> bool {
        self.simulation
    }

    /// Initialize a transaction. Returns an `access_code` for in-app Inline checkout.
    pub async fn initialize_transaction(
        &self,
        email: &str,
        amount_kobo: u64,
        plan_name: &str,
    ) -> Result<PaystackInitResult, PaystackError> {
        let now_secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // Local simulation fallback for testing without external network credentials
        if self.is_simulation() {
            let seq = SIM_TX_COUNTER.fetch_add(1, Ordering::Relaxed);
            let reference = format!("starter_sim_{}_{:x}", now_secs, seq);
            let transactions = SIM_TRANSACTIONS.get_or_init(|| Mutex::new(Default::default()));
            if let Ok(mut transactions) = transactions.lock() {
                transactions.insert(reference.clone(), amount_kobo);
            }
            let access_code = format!("sim_acc_{:x}", seq);
            let authorization_url = format!("https://checkout.paystack.com/sim_{:x}", seq);

            tracing::info!(
                "Paystack [Simulation]: initialized transaction {} for {} (amount: {} kobo)",
                reference,
                email,
                amount_kobo
            );

            return Ok(PaystackInitResult {
                access_code,
                reference,
                amount_kobo,
                authorization_url,
            });
        }

        let secret_key = self.secret_key.as_ref().ok_or_else(|| {
            PaystackError::Api(
                "PAYSTACK_SECRET_KEY is required; use PAYSTACK_MODE=simulation to opt in to the test simulator".to_string(),
            )
        })?;
        let metadata = serde_json::json!({
            "plan_name": plan_name,
            "source": "dioxus_cross_platform_starter",
            "initiated_at": now_secs,
        });

        let payload = InitializePayload {
            email,
            amount: amount_kobo,
            currency: "NGN",
            metadata,
        };

        let res = self
            .http
            .post("https://api.paystack.co/transaction/initialize")
            .header("Authorization", format!("Bearer {}", secret_key))
            .header("User-Agent", paystack_user_agent())
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;

        let status = res.status();
        let body: PaystackInitResponse = res.json().await?;

        if !status.is_success() || !body.status {
            return Err(PaystackError::Api(body.message));
        }

        let data = body.data.ok_or_else(|| {
            PaystackError::Api("Paystack returned empty data object in initialization".to_string())
        })?;

        let authorization_url = data
            .authorization_url
            .unwrap_or_else(|| format!("https://checkout.paystack.com/{}", data.access_code));

        tracing::info!("Paystack: initialized live transaction {}", data.reference);

        Ok(PaystackInitResult {
            access_code: data.access_code,
            reference: data.reference,
            amount_kobo,
            authorization_url,
        })
    }

    /// Verify a transaction reference with Paystack.
    pub async fn verify_transaction(
        &self,
        reference: &str,
    ) -> Result<PaymentVerificationResult, PaystackError> {
        // If simulation mode or reference is a simulation token
        if self.is_simulation() {
            let transactions = SIM_TRANSACTIONS.get_or_init(|| Mutex::new(Default::default()));
            let amount_kobo = transactions
                .lock()
                .map_err(|error| PaystackError::Api(error.to_string()))?
                .get(reference)
                .copied()
                .ok_or_else(|| PaystackError::Api("Unknown simulated transaction".to_string()))?;
            tracing::info!("Paystack [Simulation]: verified transaction {}", reference);
            return Ok(PaymentVerificationResult {
                reference: reference.to_string(),
                is_paid: true,
                amount_kobo,
                channel: "card (simulated)".to_string(),
                message: "Simulated payment verified successfully".to_string(),
                paid_at: Some("2026-09-29T10:00:00Z".to_string()),
            });
        }

        let secret_key = self.secret_key.as_ref().ok_or_else(|| {
            PaystackError::Api(
                "PAYSTACK_SECRET_KEY is required; use PAYSTACK_MODE=simulation to opt in to the test simulator".to_string(),
            )
        })?;
        let url = format!("https://api.paystack.co/transaction/verify/{}", reference);

        let res = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", secret_key))
            .header("User-Agent", paystack_user_agent())
            .send()
            .await?;

        let status = res.status();
        let body: PaystackVerifyResponse = res.json().await?;

        if !status.is_success() || !body.status {
            return Err(PaystackError::Api(body.message));
        }

        let data = body.data.ok_or_else(|| {
            PaystackError::Api("Paystack returned empty data payload in verification".to_string())
        })?;

        let is_paid = data.status.eq_ignore_ascii_case("success");
        let channel = data.channel.unwrap_or_else(|| "unknown".to_string());
        let message = data.gateway_response.unwrap_or(body.message);

        tracing::info!(
            "Paystack: verified reference {} - is_paid: {}, channel: {}",
            data.reference,
            is_paid,
            channel
        );

        Ok(PaymentVerificationResult {
            reference: data.reference,
            is_paid,
            amount_kobo: data.amount,
            channel,
            message,
            paid_at: data.paid_at,
        })
    }

    /// Verify the integrity and authenticity of incoming Paystack webhook requests.
    /// Computes HMAC-SHA512 of payload using secret key and performs constant-time validation.
    pub fn verify_webhook_signature(&self, payload: &[u8], signature_hex: &str) -> bool {
        let Some(secret_key) = &self.secret_key else {
            return false;
        };

        verify_hmac_sha512(payload, signature_hex, secret_key)
    }
}

/// Decode a hex-encoded string into raw bytes.
fn decode_hex(s: &str) -> Option<Vec<u8>> {
    let clean = s.trim();
    if !clean.len().is_multiple_of(2) {
        return None;
    }
    (0..clean.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&clean[i..i + 2], 16).ok())
        .collect()
}

/// Constant-time verification of HMAC-SHA512 signature.
pub fn verify_hmac_sha512(payload: &[u8], signature_hex: &str, secret_key: &str) -> bool {
    let key = hmac::Key::new(hmac::HMAC_SHA512, secret_key.as_bytes());
    if let Some(expected_sig) = decode_hex(signature_hex) {
        hmac::verify(&key, payload, &expected_sig).is_ok()
    } else {
        false
    }
}

/// Initialize payment using the global/environment Paystack client.
pub async fn initialize_payment(
    email: &str,
    amount_kobo: u64,
    plan_name: &str,
) -> Result<PaystackInitResult, PaystackError> {
    let client = PaystackClient::from_env();
    client
        .initialize_transaction(email, amount_kobo, plan_name)
        .await
}

/// Verify payment reference using the global/environment Paystack client.
pub async fn verify_payment(reference: &str) -> Result<PaymentVerificationResult, PaystackError> {
    let client = PaystackClient::from_env();
    client.verify_transaction(reference).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_simulation_flow() {
        let client = PaystackClient::new("sim_test_key");
        assert!(client.is_simulation());

        let init = client
            .initialize_transaction("test@example.com", 250_000, "Example Basic Plan")
            .await
            .expect("Initialization should succeed in simulation mode");

        assert!(init.access_code.starts_with("sim_acc_"));
        assert!(init.reference.starts_with("starter_sim_"));
        assert_eq!(init.amount_kobo, 250_000);
        assert!(
            init.authorization_url
                .starts_with("https://checkout.paystack.com/sim_")
        );

        let verify = client
            .verify_transaction(&init.reference)
            .await
            .expect("Verification should succeed in simulation mode");

        assert!(verify.is_paid);
        assert_eq!(verify.reference, init.reference);
        assert_eq!(verify.amount_kobo, init.amount_kobo);
        assert_eq!(verify.channel, "card (simulated)");
        assert!(
            client
                .verify_transaction("missing-reference")
                .await
                .is_err()
        );
    }

    #[tokio::test]
    async fn missing_secret_key_does_not_simulate_payment() {
        let client = PaystackClient::new("");
        assert!(!client.is_simulation());
        assert!(
            client
                .initialize_transaction("customer@example.com", 100, "Example")
                .await
                .is_err()
        );
        assert!(client.verify_transaction("reference").await.is_err());
    }

    #[test]
    fn test_hmac_sha512_verification() {
        let secret = "sk_test_secret_key_12345";
        let payload = b"{\"event\":\"charge.success\",\"data\":{\"reference\":\"ref_1001\"}}";

        // Generate valid signature using ring::hmac
        let key = hmac::Key::new(hmac::HMAC_SHA512, secret.as_bytes());
        let tag = hmac::sign(&key, payload);
        let valid_hex: String = tag.as_ref().iter().map(|b| format!("{:02x}", b)).collect();

        assert!(verify_hmac_sha512(payload, &valid_hex, secret));
        assert!(!verify_hmac_sha512(payload, "invalid_sig_hex", secret));
        assert!(!verify_hmac_sha512(b"tampered payload", &valid_hex, secret));
        assert!(!verify_hmac_sha512(payload, &valid_hex, "wrong_secret"));
    }

    #[test]
    fn test_hex_decoding() {
        assert_eq!(decode_hex("00ff10"), Some(vec![0x00, 0xff, 0x10]));
        assert_eq!(decode_hex(""), Some(vec![]));
        assert_eq!(decode_hex("1"), None); // Odd length
        assert_eq!(decode_hex("zz"), None); // Invalid hex
    }

    #[tokio::test]
    #[ignore]
    async fn test_live_paystack_env_initialization() {
        let client = PaystackClient::from_env();
        if client.is_simulation() {
            println!("Skipping live test: PAYSTACK_SECRET_KEY not set or in simulation mode");
            return;
        }

        let init = client
            .initialize_transaction("customer@example.com", 250_000, "Example Basic Plan")
            .await
            .expect("Live Paystack initialization should succeed");

        println!("Live Init Result: {:?}", init);
        assert!(!init.access_code.is_empty());
        assert!(!init.reference.is_empty());
        assert!(
            init.authorization_url
                .starts_with("https://checkout.paystack.com/")
        );
    }
}
