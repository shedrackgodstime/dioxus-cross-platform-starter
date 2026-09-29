use dioxus::prelude::*;
use starter_core::{PaymentVerificationResult, PaystackInitResult, ServerStatus};

/// Health and diagnostic server function.
/// On client targets: Compiles to an HTTP GET request to `/api/v1/status`.
/// On server targets: Compiles to an Axum handler calling `starter_server::get_status()`.
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

/// Initiate a Paystack payment session.
/// Returns the `access_code` and transaction `reference`.
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

/// Verify a completed or submitted Paystack payment reference.
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
