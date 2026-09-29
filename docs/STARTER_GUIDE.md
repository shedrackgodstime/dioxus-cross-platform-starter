# Starter Workspace Guide

This workspace keeps the recurring setup for full-stack Dioxus projects in one place. Product behavior belongs in `core`, `server`, and project-specific UI modules; platform bootstrapping and cross-platform adapters stay reusable here.

## Crate responsibilities

| Crate | Owns | Must not own |
| --- | --- | --- |
| `core` | Pure Rust domain primitives, validation, and capability traits | Dioxus, platform APIs, filesystem, networking |
| `api` | DTOs and typed Dioxus server functions | UI or platform renderer details |
| `server` | Backend services and third-party provider clients | UI components or native launchers |
| `ui` | Shared components, routes, pages, reactive app state | Backend internals and platform-specific storage |
| `web` | Web client and full-stack server entrypoint | Desktop/mobile launch configuration |
| `desktop` | Desktop launch, window configuration, desktop adapters | Web/mobile launch code |
| `mobile` | Android/iOS launch, mobile adapters and lifecycle setup | Web/desktop launch code |

Run `make check-boundaries` after adding a crate or changing a dependency. Add its intended dependency rules to `scripts/check-boundaries.py` at the same time.

## Local setup

1. Install the pinned Rust toolchain and Dioxus CLI `0.7.10`.
2. Run `dx doctor` to identify missing platform tools.
3. Copy `.env.example` to `.env`; keep secrets there and out of client build variables.
4. Install the targets for the platforms you use. Android requires the SDK/NDK; iOS requires macOS and Xcode.
5. Run `make check-all`, then start the target with the matching `make serve-*` command.

Dioxus builds the same UI against distinct platform renderers and Cargo feature sets. Keep platform-only dependencies in the launcher crate or target-specific dependency sections. Dioxus's [platform guide](https://dioxuslabs.com/learn/0.7/guides/platforms/) describes the renderer differences and setup requirements.

## Configuration

- `SERVER_URL` overrides the server endpoint for a Make target.
- `PROD_SERVER_URL` supplies the release endpoint.
- `DEV_DESKTOP_URL` configures local web/desktop development.
- `DEV_ANDROID_URL` defaults to Android emulator host routing (`10.0.2.2`). Physical devices need a reachable LAN address.
- `ANDROID_HOME` and `ANDROID_NDK_HOME` can point to a non-default SDK/NDK installation.
- `PAYSTACK_SECRET_KEY` is server-only. Client builds may receive a server URL, never the secret key.

Configuration should have one documented source. When changing a variable, update `.env.example`, Make help, and this section together.

## Platform adapter rules

- Keep UI code on the `KeyValueStore` capability interface; implement storage in the launcher.
- Unsupported or unavailable storage must return an error. Do not silently substitute temporary storage for persistent storage.
- Keep shared route variants unconditional. Use platform-specific code for adapter and launcher behavior where needed.
- A target is supported only when CI compiles it and a manual simulator/device smoke check covers launch, storage, navigation, server calls, and packaging.
- Dioxus desktop and mobile render through system webviews. Verify layout, keyboard behavior, deep links, permissions, and external browser handoffs on actual target environments.

## Paystack integration

The checkout screen is an integration example, not a product catalog. The example uses server-side product IDs to look up prices so the browser cannot submit an arbitrary amount. Replace the example product catalog and connect order ownership, expected amount/currency checks, idempotency, and fulfillment to the consuming application's server logic.

The inline bridge is shared UI code, but Paystack documents InlineJS as a web JavaScript library. WebView behavior must be smoke-tested independently on desktop, Android, and iOS; retain an explicit fallback where InlineJS is unavailable. See [PAYSTACK_INTEGRATION.md](PAYSTACK_INTEGRATION.md).

## Quality gates

GitHub Actions runs formatting, crate-boundary checks, workspace and WASM checks, tests, Clippy, and Android/iOS target checks. Local equivalents:

```sh
make fmt-check
make check-boundaries
cargo check --workspace --locked
cargo check --package starter-web --target wasm32-unknown-unknown --locked
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Cross-compilation does not replace UI smoke tests, release signing, or verifying the generated artifacts on each operating system.
