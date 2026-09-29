#[cfg(feature = "fullstack")]
use dioxus::prelude::*;
use starter_core::{PaymentVerificationResult, PaystackInitResult, ServerStatus};

#[cfg(feature = "web-client")]
use gloo_net::http::Request;
#[cfg(feature = "web-client")]
use std::sync::OnceLock;

#[cfg(feature = "web-client")]
static CLIENT_BASE_URL: OnceLock<String> = OnceLock::new();

#[cfg(feature = "web-client")]
pub fn set_client_base_url(url: String) {
    let _ = CLIENT_BASE_URL.set(url.trim_end_matches('/').to_string());
}

#[cfg(feature = "web-client")]
fn client_url(path: &str) -> String {
    format!(
        "{}{}",
        CLIENT_BASE_URL.get().map(String::as_str).unwrap_or(""),
        path
    )
}

#[cfg(feature = "web-client")]
async fn decode<T: serde::de::DeserializeOwned>(
    response: gloo_net::http::Response,
) -> Result<T, String> {
    if !response.ok() {
        return Err(format!("backend returned HTTP {}", response.status()));
    }
    response.json().await.map_err(|error| error.to_string())
}

/// Health and diagnostic server function.
/// On client targets: Compiles to an HTTP GET request to `/api/v1/status`.
/// On server targets: Compiles to an Axum handler calling `starter_server::get_status()`.
#[cfg(feature = "fullstack")]
#[get("/api/v1/status")]
pub async fn get_server_status() -> Result<ServerStatus, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(starter_server::get_status().await)
    }
    #[cfg(not(feature = "server"))]
    {
        unreachable!()
    }
}

#[cfg(feature = "web-client")]
pub async fn get_server_status() -> Result<ServerStatus, String> {
    decode(
        Request::get(&client_url("/api/v1/status"))
            .send()
            .await
            .map_err(|error| error.to_string())?,
    )
    .await
}

/// Initiate a Paystack payment session.
/// Returns the `access_code` and transaction `reference`.
#[cfg(feature = "fullstack")]
#[post("/api/v1/payment/initialize")]
pub async fn initiate_paystack_payment(
    email: String,
    product_id: String,
) -> Result<PaystackInitResult, ServerFnError> {
    #[cfg(feature = "server")]
    {
        let product = starter_core::PaymentPlan::from_product_id(&product_id)
            .ok_or_else(|| ServerFnError::new("Unknown checkout product"))?;
        starter_server::paystack::initialize_payment(&email, product.amount_kobo(), product.name())
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = (email, product_id);
        unreachable!()
    }
}

#[cfg(feature = "web-client")]
pub async fn initiate_paystack_payment(
    email: String,
    product_id: String,
) -> Result<PaystackInitResult, String> {
    decode(
        Request::post(&client_url("/api/v1/payment/initialize"))
            .json(&serde_json::json!({ "email": email, "product_id": product_id }))
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?,
    )
    .await
}

/// Verify a completed or submitted Paystack payment reference.
#[cfg(feature = "fullstack")]
#[post("/api/v1/payment/verify")]
pub async fn verify_paystack_payment(
    reference: String,
) -> Result<PaymentVerificationResult, ServerFnError> {
    #[cfg(feature = "server")]
    {
        starter_server::paystack::verify_payment(&reference)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = reference;
        unreachable!()
    }
}

#[cfg(feature = "web-client")]
pub async fn verify_paystack_payment(
    reference: String,
) -> Result<PaymentVerificationResult, String> {
    decode(
        Request::post(&client_url("/api/v1/payment/verify"))
            .json(&serde_json::json!({ "reference": reference }))
            .map_err(|error| error.to_string())?
            .send()
            .await
            .map_err(|error| error.to_string())?,
    )
    .await
}
