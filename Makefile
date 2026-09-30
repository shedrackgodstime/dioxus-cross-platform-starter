SHELL := /bin/sh

CARGO ?= cargo
DX ?= dx

# The API and the standalone web client are separate processes, so they need
# separate ports. PORT is canonical: it is what `core::config` falls back to
# and what `dioxus::serve` binds for the server and fullstack targets.
PORT ?= 8080
WEB_PORT ?= 8081

# API base baked into the web and native clients at compile time.
DEV_SERVER_API_URL ?= http://10.101.124.151:$(PORT)
DEV_SERVER_BIND ?= 0.0.0.0:$(PORT)
ANDROID_TARGET ?= aarch64-linux-android
ANDROID_DEVICE ?=
ANDROID_PACKAGE ?= com.example.Mobile
ANDROID_APK ?= target/dx/mobile/release/android/app/app/build/outputs/apk/debug/app-debug.apk

export DEV_SERVER_API_URL

ifneq ($(strip $(SERVER_API_URL)),)
export SERVER_API_URL
endif

.DEFAULT_GOAL := help

.PHONY: help \
	serve-server serve-web serve-fullstack \
	serve-android serve-ios \
	serve-linux serve-windows serve-macos \
	build-server build-web build-fullstack build \
	build-android build-ios install-android \
	build-linux build-windows build-macos \
	fmt check test quality verify clean

## Runtime targets

## serve-server    Run the API server only (no web client or UI) on $(PORT).
serve-server:
	PORT=$(PORT) $(CARGO) run -p server --offline

## serve-web       Run the standalone web client only on $(WEB_PORT).
##                 Run `make serve-server` too; the client calls the API at
##                 $(DEV_SERVER_API_URL).
serve-web:
	$(DX) serve --package web --port $(WEB_PORT) --addr $(shell echo $(DEV_SERVER_BIND) | cut -d: -f1)

## serve-fullstack Run the web client and fullstack server together on $(PORT).
serve-fullstack:
	$(DX) serve --package fullstack --addr $(shell echo $(DEV_SERVER_BIND) | cut -d: -f1) --port $(shell echo $(DEV_SERVER_BIND) | cut -d: -f2)

## serve-android   Run the shared native client on Android.
serve-android:
	@device="$(ANDROID_DEVICE)"; \
	if [ -z "$$device" ] && command -v adb >/dev/null 2>&1; then \
		device=$$(adb devices | awk 'NR > 1 && $$2 == "device" { print $$1; exit }'); \
	fi; \
	if [ -z "$$device" ]; then \
		echo "No Android device found. Connect a device or set ANDROID_DEVICE=<serial>."; \
		exit 1; \
	fi; \
	$(DX) serve --package mobile --platform android --target $(ANDROID_TARGET) --device "$$device"

## serve-ios       Run the shared native client on iOS.
serve-ios:
	$(DX) serve --package mobile --platform ios

## serve-linux     Run the shared desktop client on Linux.
serve-linux:
	$(DX) serve --package desktop --platform linux

## serve-windows   Run the shared desktop client on Windows.
serve-windows:
	$(DX) serve --package desktop --platform windows

## serve-macos     Run the shared desktop client on macOS.
serve-macos:
	$(DX) serve --package desktop --platform macos

## Release build targets

## build-server    Build the API server in release mode.
build-server:
	$(CARGO) build --release -p server --offline

## build-web       Build the standalone web client in release mode.
build-web:
	$(DX) build --release --package web --platform web

## build-fullstack Build the fullstack client and server bundle in release mode.
build-fullstack:
	$(DX) build --release --package fullstack --platform web

## build-android   Build the native client for Android in release mode.
build-android:
	$(DX) build --release --package mobile --platform android --target $(ANDROID_TARGET)

## install-android Build, install, and launch the Android release on ADB.
install-android: build-android
	@device="$(ANDROID_DEVICE)"; \
	if [ -z "$$device" ] && command -v adb >/dev/null 2>&1; then \
		device=$$(adb devices | awk 'NR > 1 && $$2 == "device" { print $$1; exit }'); \
	fi; \
	if [ -z "$$device" ]; then \
		echo "No Android device found. Connect a device or set ANDROID_DEVICE=<serial>."; \
		exit 1; \
	fi; \
	if [ ! -f "$(ANDROID_APK)" ]; then \
		echo "Android APK not found: $(ANDROID_APK)"; \
		exit 1; \
	fi; \
	echo "Installing $(ANDROID_APK) on $$device..."; \
	adb -s "$$device" install --no-incremental -r "$(ANDROID_APK)"; \
	echo "Launching $(ANDROID_PACKAGE)..."; \
	adb -s "$$device" shell monkey -p "$(ANDROID_PACKAGE)" 1 >/dev/null

## build-ios       Build the native client for iOS in release mode.
build-ios:
	$(DX) build --release --package mobile --platform ios

## build-linux     Build the desktop client for Linux in release mode.
build-linux:
	$(DX) build --release --package desktop --platform linux

## build-windows   Build the desktop client for Windows in release mode.
build-windows:
	$(DX) build --release --package desktop --platform windows

## build-macos     Build the desktop client for macOS in release mode.
build-macos:
	$(DX) build --release --package desktop --platform macos

## build           Build all targets supported by the current host.
build: build-server build-web build-fullstack build-android

ifeq ($(shell uname -s),Darwin)
build: build-ios build-macos
else ifeq ($(shell uname -s),Linux)
build: build-linux
else ifneq (,$(findstring NT,$(shell uname -s)))
build: build-windows
endif

## Quality target

## fmt             Format all Rust code.
fmt:
	$(CARGO) fmt --all

## check           Check every workspace crate.
check:
	$(CARGO) check --workspace --offline

## test            Run every workspace test.
test:
	$(CARGO) test --workspace --offline

## quality         Run format, check, and test together.
quality: fmt check test

## verify          Alias for the complete quality check.
verify: quality

## clean           Remove Cargo build artifacts.
clean:
	$(CARGO) clean

## help            Show available commands.
help:
	@awk 'BEGIN { print "Usage: make <command>\n" } /^## [a-z]/ { line = substr($$0, 4); match(line, /^[^[:space:]]+/); command = substr(line, RSTART, RLENGTH); description = substr(line, RLENGTH + 1); sub(/^[[:space:]]+/, "", description); printf "  %-16s %s\n", command, description }' $(MAKEFILE_LIST)
