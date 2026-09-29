# Default target executed when running `make` with no arguments
.DEFAULT_GOAL := help
default: help

# Canonical server networking configuration
# ============================================================================
# 🌐 NETWORK CONFIGURATION
# ============================================================================
# Local development port
PORT ?= 8080

# Keep all dx-launched Cargo commands consistent with Cargo.lock by default.
DX_LOCKED ?= --locked

# ----------------------------------------------------------------------------
# 1. Server Bind IPs (Where backend listens on host)
# ----------------------------------------------------------------------------
# Dev: binds all interfaces (LAN & emulators can reach)
DEV_BIND_IP  ?= 0.0.0.0
# ----------------------------------------------------------------------------
# 2. Client API Endpoints (Where clients send requests)
# ----------------------------------------------------------------------------
# Production domain: Default endpoint baked into ALL release client builds
PROD_SERVER_URL ?=

# Local dev endpoints: Default endpoints for debug / serve client runs
DEV_DESKTOP_URL ?= http://localhost:$(PORT)
DEV_ANDROID_URL ?= http://10.0.2.2:$(PORT)

# ----------------------------------------------------------------------------
# 3. Universal Override (Optional CLI flag)
# If provided, this URL overrides BOTH dev and release defaults for any target!
# Examples:
#   make build-android SERVER_URL=http://192.168.1.50:8080  (test release on phone/LAN)
#   make build-android SERVER_URL=http://10.0.2.2:8080      (test release on emulator)
#   make serve-desktop SERVER_URL=$(PROD_SERVER_URL)       (test dev against prod API)
# ----------------------------------------------------------------------------
SERVER_URL ?=

# Resolved endpoints based on whether SERVER_URL override is provided:
CLIENT_PROD_URL    := $(if $(SERVER_URL),$(SERVER_URL),$(PROD_SERVER_URL))
CLIENT_DESKTOP_URL := $(if $(SERVER_URL),$(SERVER_URL),$(DEV_DESKTOP_URL))
CLIENT_ANDROID_URL := $(if $(SERVER_URL),$(SERVER_URL),$(DEV_ANDROID_URL))

# Android NDK configuration
ANDROID_HOME ?= $(if $(ANDROID_SDK_ROOT),$(ANDROID_SDK_ROOT),$(HOME)/Android/Sdk)
ANDROID_NDK_HOME ?= $(lastword $(sort $(wildcard $(ANDROID_HOME)/ndk/*)))
ifeq ($(origin ANDROID_HOST_TAG),undefined)
ifeq ($(shell uname -s),Darwin)
ifeq ($(shell uname -m),arm64)
ANDROID_HOST_TAG := darwin-arm64
else
ANDROID_HOST_TAG := darwin-x86_64
endif
else
ANDROID_HOST_TAG := linux-x86_64
endif
endif
ANDROID_LLVM_BIN ?= $(ANDROID_NDK_HOME)/toolchains/llvm/prebuilt/$(ANDROID_HOST_TAG)/bin
ANDROID_TARGET ?= aarch64-linux-android
ANDROID_ENV := ANDROID_HOME=$(ANDROID_HOME) ANDROID_NDK_HOME=$(ANDROID_NDK_HOME) PATH="$(ANDROID_LLVM_BIN):$(PATH)"

# Release signing parameters (Android keystore)
APKSIGNER         ?= $(firstword $(wildcard $(ANDROID_HOME)/build-tools/*/apksigner apksigner))
KEYSTORE_PATH     ?= release.keystore
KEYSTORE_PASSWORD ?=
KEY_ALIAS         ?= starter
KEY_PASSWORD      ?=

help:
	@echo "=========================================================================================="
	@echo " Dioxus Cross-Platform Starter (Web, Desktop, Mobile & Fullstack)"
	@echo "=========================================================================================="
	@echo "1. Web Alone (Pure Client SPA):"
	@echo "  make serve-web           - Run standalone Web client with API base $(CLIENT_DESKTOP_URL)"
	@echo "  make build-web           - Build standalone Web SPA bundle (Release -> Points to $(CLIENT_PROD_URL))"
	@echo "  make build-web-debug     - Build standalone Web SPA bundle (Debug -> Points to $(CLIENT_DESKTOP_URL))"
	@echo ""
	@echo "2. Web + Server Together (Fullstack SSR + Server Functions):"
	@echo "  make serve-fullstack     - Run Fullstack Web + Server with Dioxus HMR on $(DEV_BIND_IP):$(PORT)"
	@echo "  make serve-web-server    - Alias for serve-fullstack"
	@echo "  make serve-server        - Run the fullstack server binary directly (no Dioxus HMR/browser watcher)"
	@echo "  make build-web-server    - Build combined Web client assets + Fullstack binary (Release)"
	@echo "  make build-web-server-debug - Build combined Web client + Fullstack binary (Debug)"
	@echo ""
	@echo "4. Native Targets (Desktop, Android, iOS):"
	@echo "  make serve-desktop       - Run Desktop app (Debug -> Points to $(CLIENT_DESKTOP_URL))"
	@echo "  make build-desktop       - Build Desktop bundle (Release -> Points to $(CLIENT_PROD_URL))"
	@echo "  make build-desktop-debug - Build Desktop bundle (Debug -> Points to $(CLIENT_DESKTOP_URL))"
	@echo "  make serve-android       - Run Android app on emulator/device (Debug -> Points to $(CLIENT_ANDROID_URL))"
	@echo "  make build-android       - Build ARM64 Android APK (Release -> Points to $(CLIENT_PROD_URL))"
	@echo "  make build-android-debug - Build ARM64 Android APK (Debug -> Points to $(CLIENT_ANDROID_URL))"
	@echo "  make install-android     - Build, install debug APK on connected device, and launch"
	@echo "  make serve-ios           - Run iOS app on simulator (Debug -> Points to $(CLIENT_DESKTOP_URL))"
	@echo "  make build-ios           - Build iOS bundle (Release -> Points to $(CLIENT_PROD_URL))"
	@echo "  make build-ios-debug     - Build iOS app (Debug -> Points to $(CLIENT_DESKTOP_URL))"
	@echo "  make build-all           - Build available targets (Web, Server, Desktop, Android)"
	@echo ""
	@echo "5. Release Signing & Diagnostics:"
	@echo "  make sign-android        - Sign release APK using apksigner with KEYSTORE_PATH"
	@echo "  make check-signing       - Check available code signing tools (apksigner, keytool, codesign)"
	@echo "  make check-all           - Verify host, full-stack, and WASM compilation"
	@echo "  make check-android       - Compile-check the Android client"
	@echo "  make check-ios           - Compile-check iOS device and simulator targets"
	@echo "  make check-desktop       - Compile-check the desktop launcher"
	@echo "  make ci-quality          - Run the complete CI quality and debug build gates"
	@echo "  make test                - Run all behavioral unit tests"
	@echo "  make fmt                 - Format Rust code"
	@echo "  make fmt-check           - Check Rust formatting"
	@echo "  make check-boundaries    - Check workspace crate boundaries"
	@echo ""
	@echo "Active Network Configuration:"
	@echo "  PORT=$(PORT)"
	@echo "  PROD_SERVER_URL=$(PROD_SERVER_URL) (Release builds connect here)"
	@echo "  DEV_DESKTOP_URL=$(DEV_DESKTOP_URL) (Desktop/Web debug connects here)"
	@echo "  DEV_ANDROID_URL=$(DEV_ANDROID_URL) (Android debug connects here)"
	@echo "  DEV_BIND_IP=$(DEV_BIND_IP) (Dev server listens on $(DEV_BIND_IP):$(PORT))"
	@echo "  SERVER_URL override: $(if $(SERVER_URL),$(SERVER_URL),(none - using respective defaults))"

# ----------------------------------------------------------------------------
# 1. Web Alone (Pure Client SPA)
# ----------------------------------------------------------------------------

serve-web:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx serve --package starter-web-client --platform web --addr $(DEV_BIND_IP) --port $(PORT) $(DX_LOCKED)

serve-web-only: serve-web

build-web: require-client-server-url
	STARTER_SERVER_URL="$(CLIENT_PROD_URL)" dx build --package starter-web-client --platform web --release $(DX_LOCKED)

build-web-debug:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx build --package starter-web-client --platform web $(DX_LOCKED)

# ----------------------------------------------------------------------------
# 2. Fullstack development modes
# ----------------------------------------------------------------------------

# Run the deployable fullstack binary directly. This does not start the Dioxus
# dev server, live reload, or browser watcher.
serve-server:
	IP=$(DEV_BIND_IP) PORT=$(PORT) cargo run --package starter-web --features server $(DX_LOCKED)

serve-server-debug: serve-server

build-server: build-web-server

build-server-debug: build-web-server-debug

# ----------------------------------------------------------------------------
# 3. Web + Server Together (Fullstack SSR + Hydrated Client Bundle)
# ----------------------------------------------------------------------------

serve-web-server:
	IP=$(DEV_BIND_IP) PORT=$(PORT) dx serve --package starter-web --addr $(DEV_BIND_IP) --port $(PORT) $(DX_LOCKED)

serve-fullstack: serve-web-server

build-web-server:
	dx build --package starter-web --platform web --fullstack=true --release $(DX_LOCKED)

build-web-server-debug:
	dx build --package starter-web --platform web --fullstack=true $(DX_LOCKED)

build-fullstack: build-web-server

# ----------------------------------------------------------------------------
# 4. Native Targets (Desktop, Android, iOS)
# ----------------------------------------------------------------------------

serve-desktop:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx serve --package starter-desktop $(DX_LOCKED)

build-desktop: require-client-server-url
	STARTER_SERVER_URL="$(CLIENT_PROD_URL)" dx build --package starter-desktop --platform desktop --release $(DX_LOCKED)

build-desktop-debug:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx build --package starter-desktop --platform desktop $(DX_LOCKED)

serve-android:
	$(ANDROID_ENV) STARTER_SERVER_URL="$(CLIENT_ANDROID_URL)" dx serve --package starter-mobile --platform android --target $(ANDROID_TARGET) $(DX_LOCKED)

serve-andriod: serve-android

build-android: require-client-server-url
	$(ANDROID_ENV) STARTER_SERVER_URL="$(CLIENT_PROD_URL)" dx build --package starter-mobile --platform android --target $(ANDROID_TARGET) --release $(DX_LOCKED)

build-android-debug:
	$(ANDROID_ENV) STARTER_SERVER_URL="$(CLIENT_ANDROID_URL)" dx build --package starter-mobile --platform android --target $(ANDROID_TARGET) $(DX_LOCKED)

install-android: build-android-debug
	adb install -r target/dx/starter-mobile/debug/android/app/app/build/outputs/apk/debug/app-debug.apk
	adb shell am start -n dev.dioxus.starter/dev.dioxus.main.MainActivity

serve-ios:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx serve --package starter-mobile --platform ios $(DX_LOCKED)

build-ios: require-client-server-url
	STARTER_SERVER_URL="$(CLIENT_PROD_URL)" dx build --package starter-mobile --platform ios --release $(DX_LOCKED)

build-ios-debug:
	STARTER_SERVER_URL="$(CLIENT_DESKTOP_URL)" dx build --package starter-mobile --platform ios $(DX_LOCKED)

build-all: build-web build-server build-desktop build-android

require-client-server-url:
	@if [ -z "$(CLIENT_PROD_URL)" ]; then \
		echo "Set PROD_SERVER_URL or SERVER_URL before building a standalone production client."; \
		exit 1; \
	fi

# ----------------------------------------------------------------------------
# 5. Release Signing & Verification
# ----------------------------------------------------------------------------

sign-android:
	@echo "Checking Android release APK..."
	@APK_PATH=$$(find target/dx/starter-mobile -name "*.apk" | grep release | head -n 1); \
	if [ -z "$$APK_PATH" ]; then \
		echo "Error: Release APK not found. Run 'make build-android' first."; \
		exit 1; \
	fi; \
	if [ ! -f "$(KEYSTORE_PATH)" ]; then \
		echo "Error: Keystore '$(KEYSTORE_PATH)' not found. See docs/RELEASE_SIGNING.md to generate one."; \
		exit 1; \
	fi; \
	echo "Signing $$APK_PATH with $(KEYSTORE_PATH)..."; \
	$(APKSIGNER) sign --ks "$(KEYSTORE_PATH)" --ks-key-alias "$(KEY_ALIAS)" $$APK_PATH && \
	echo "APK signed successfully! Verifying..." && \
	$(APKSIGNER) verify --verbose $$APK_PATH

check-signing:
	@echo "Checking code signing tools..."
	@if [ -x "$(APKSIGNER)" ] || command -v $(APKSIGNER) >/dev/null 2>&1; then \
		echo "  [✓] apksigner (Android) found: $(APKSIGNER)"; \
	else \
		echo "  [ ] apksigner not found"; \
	fi
	@which keytool >/dev/null 2>&1 && echo "  [✓] keytool (JDK) found: $$(which keytool)" || echo "  [ ] keytool not found in PATH"
	@which codesign >/dev/null 2>&1 && echo "  [✓] codesign (macOS/iOS) found: $$(which codesign)" || echo "  [ ] codesign (macOS/iOS) not found"
	@which signtool >/dev/null 2>&1 && echo "  [✓] signtool (Windows) found" || echo "  [ ] signtool (Windows) not found"

# ----------------------------------------------------------------------------
# Diagnostics & Tests
# ----------------------------------------------------------------------------

check-all:
	@echo "--> Checking Host workspace..."
	cargo check --workspace --locked
	@echo "--> Checking full-stack server feature..."
	cargo check --package starter-web --features server --locked
	@echo "--> Checking WASM web client..."
	cargo check --package starter-web --target wasm32-unknown-unknown --locked
	@echo "--> Checking standalone WASM web client..."
	cargo check --package starter-web-client --target wasm32-unknown-unknown --locked

check-android:
	@echo "--> Checking Android mobile client..."
	$(ANDROID_ENV) cargo check --package starter-mobile --target $(ANDROID_TARGET) --locked

check-ios:
	@echo "--> Checking iOS device client..."
	cargo check --package starter-mobile --target aarch64-apple-ios --locked
	@echo "--> Checking iOS simulator client..."
	cargo check --package starter-mobile --target aarch64-apple-ios-sim --locked

check-desktop:
	@echo "--> Checking desktop launcher..."
	cargo check --package starter-desktop --locked

clippy:
	cargo clippy --workspace --all-targets --locked -- -D warnings
	cargo clippy --package starter-web --features server --locked -- -D warnings

ci-quality: fmt-check check-boundaries check-all test clippy

test:
	cargo test --workspace --locked

fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

check-boundaries:
	python3 scripts/check-boundaries.py
