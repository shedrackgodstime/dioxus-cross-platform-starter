# Paystack Inline Integration

This integration keeps Paystack's secret key on the server, initializes a transaction through a typed Dioxus server function, hands the returned access code to InlineJS, then verifies the transaction through the server. The checkout page is a removable usage example.

Paystack describes `resumeTransaction(accessCode)` as the way to complete a server-initialized transaction in the browser. See the official [InlineJS guide](https://paystack.com/docs/developer-tools/inlinejs/) and [accept payments guide](https://paystack.com/docs/payments/accept-payments/).

## Flow in this workspace

1. The UI selects an example product ID and sends it with the customer's email.
2. The `api` server function maps the ID to the example price on the server. It does not accept an amount from the client.
3. The `server` Paystack client calls the transaction initialize endpoint using `PAYSTACK_SECRET_KEY`.
4. The UI passes the returned access code to `PaystackPop.resumeTransaction` through Dioxus document evaluation.
5. InlineJS reports the transaction reference to Rust; the UI asks the server to verify that reference.
6. The UI displays the verification result. This starter does not fulfill orders or grant entitlements.

The sample products live in `starter_core::PaymentPlan`. Replace those examples with a lookup against the consuming application's authoritative catalog/order store. Never trust a price, order owner, or entitlement sent only by the client.

## Configuration

```dotenv
PAYSTACK_SECRET_KEY=sk_test_...
# Optional; simulation is disabled unless explicitly selected.
# PAYSTACK_MODE=simulation
```

Copy `.env.example` for local configuration. `.env` is ignored by Git. `dotenvy` loads it for the full-stack server; do not set `PAYSTACK_SECRET_KEY` as a client build variable or compile-time environment value.

Simulation is opt-in and enabled only in debug builds. It returns simulated successes and must only be used for local development. Release builds ignore the simulation setting and require a real secret key. Live integration tests are ignored by default; do not run them against production credentials as part of CI.

## InlineJS and platform support

The SDK is loaded on demand by the checkout flow rather than for every app launch. If InlineJS fails to load or `PaystackPop` is unavailable, the UI keeps a manual verification/fallback path.

Paystack documents InlineJS for browser applications. The starter also packages the shared UI in desktop/mobile webviews, but that alone does not establish supported payment behavior on those platforms. Before enabling checkout in a project, smoke-test the Inline modal, cancellation, success callback, app resume, external bank-app handoff, and fallback on each target webview and OS version. If a native target is unreliable, use its supported external checkout or native SDK flow behind a platform adapter.

## Production requirements for consuming projects

- Resolve the price and currency from trusted server-side data for the authenticated customer/order.
- Persist the order and expected amount before initialization; use an order-linked unique reference/idempotency strategy.
- On verification, require success and compare reference, expected amount, currency, and order ownership before fulfillment.
- Treat browser callbacks as a signal to verify, never as proof of payment.
- Add a webhook endpoint and verify the `x-paystack-signature` HMAC using the raw request body for asynchronous payment methods.
- Make fulfillment idempotent so callback and webhook retries cannot grant value more than once.
- Keep secret keys in deployment secret storage; rotate any key accidentally committed or shared.

Paystack explicitly requires server-side initialization and verification, and advises checking the verified amount before delivering value. See [Accept Payments](https://paystack.com/docs/payments/accept-payments/) and [Verify Payments](https://paystack.com/docs/payments/verify-payments/).
