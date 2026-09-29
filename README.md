# Dioxus Cross-Platform Starter

A personal Cargo workspace for starting full-stack Dioxus projects with web, desktop, Android, and iOS launchers. The shared UI and domain code stay reusable; platform adapters and backend-only dependencies remain at their boundaries.

## Workspace

```text
crates/
├── core/      # Pure Rust domain types, validation, and capability ports
├── api/       # Typed Dioxus server functions and shared DTOs
├── server/    # Backend services and provider integrations
├── ui/        # Shared components, pages, routes, and application state
├── web/       # Web and full-stack server entrypoint
├── web-client/ # Standalone browser-only web entrypoint
├── desktop/   # Desktop entrypoint and native adapters
└── mobile/    # Android/iOS entrypoint and native adapters
```

Paystack is kept as an optional integration example because its server initialization, Inline handoff, and verification flow are useful to reuse. The checkout page's catalog is illustrative. A real project must resolve the payable amount from trusted server-side order data and connect successful verification to its own fulfillment logic.

## Requirements

- Rust `1.97.1`, pinned in `rust-toolchain.toml`
- Dioxus CLI `0.7.10`
- WebAssembly target for web builds
- Android SDK/NDK for Android builds
- macOS and Xcode for iOS builds

Run `dx doctor` to inspect the local platform toolchains. The official Dioxus platform guide documents renderer-specific setup and limitations: <https://dioxuslabs.com/learn/0.7/guides/platforms/>.

Tailwind is compiled by Dioxus CLI from the root `tailwind.css` into `crates/ui/assets/tailwind.css`; `Dioxus.toml` keeps those paths shared by all launchers.

Branding assets live in `assets/icons/`. The PNG, ICO, and ICNS set is wired into `Dioxus.toml` so desktop bundles use the same launcher identity; the web entrypoint also publishes a favicon and install manifest.

Theme preference is shared through `starter-core::ThemeMode`, persisted through the platform storage port, and exposed in the Settings page. Notification behavior is defined by the core `NotificationService` port; each native launcher can add its OS adapter and report permission or availability errors explicitly.

The workspace keeps standalone web and full-stack web builds separate. `make build-web` produces only the browser client from `starter-web-client`; `make build-web-server` produces the browser client and server executable from `starter-web`.

## Common commands

```sh
make serve-web             # Standalone browser client; configure its remote API with SERVER_URL
make serve-fullstack       # Full-stack web app with SSR/server functions and Dioxus HMR
make serve-server          # Full-stack server process without the Dioxus dev server
make serve-desktop
make serve-android
make serve-ios             # Requires macOS/Xcode
make check-all
make check-boundaries
make fmt-check
make test
```

Use `SERVER_URL`, `DEV_ANDROID_URL`, `ANDROID_HOME`, and `ANDROID_NDK_HOME` to configure local environments. The Android emulator default is `10.0.2.2`; a physical mobile device needs a host address reachable on its network. Standalone production client builds require `PROD_SERVER_URL` or an explicit `SERVER_URL`; full-stack web builds use the same origin.

## Configuration and secrets

Copy `.env.example` to `.env` for local server configuration. `.env` is ignored by Git. Keep `PAYSTACK_SECRET_KEY` on the server; never bake it into browser, desktop, or mobile builds. The simulator is disabled by default, requires `PAYSTACK_MODE=simulation`, and only runs in debug builds.

## Quality gates

GitHub Actions checks formatting, crate boundaries, workspace and WASM compilation, tests, Clippy, Android ARM64, and iOS ARM64. Native UI behavior still needs device/simulator smoke checks; a successful cross-compile does not validate WebView behavior or packaging/signing.

See [STARTER_GUIDE.md](docs/STARTER_GUIDE.md), [PAYSTACK_INTEGRATION.md](docs/PAYSTACK_INTEGRATION.md), and [RELEASE_SIGNING.md](docs/RELEASE_SIGNING.md) for the workspace runbook, payment integration, and release signing.
