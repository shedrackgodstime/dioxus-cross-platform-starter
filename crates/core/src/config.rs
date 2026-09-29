use crate::models::{MobileOs, PlatformTarget};
use serde::{Deserialize, Serialize};

/// Canonical server networking defaults.
pub const DEFAULT_SERVER_PORT: u16 = 8080;
pub const DEFAULT_LOCALHOST_URL: &str = "http://127.0.0.1:8080";
/// Single source of truth debug server URL for all native targets.
pub const DEBUG_SERVER_URL: &str = "http://10.0.2.2:8080";

/// Validate and normalize an API base URL supplied by a launcher or build.
/// Relative paths are supported for same-origin fullstack deployments.
pub fn validate_server_url(value: &str) -> Result<String, String> {
    let value = value.trim();
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err("API server URL must be non-empty and contain no whitespace".into());
    }
    if value.starts_with('/') && !value.starts_with("//") {
        return Ok(value.trim_end_matches('/').to_string());
    }
    if !(value.starts_with("http://") || value.starts_with("https://")) {
        return Err("API server URL must use http://, https://, or a same-origin path".into());
    }
    let authority = value
        .split_once("://")
        .and_then(|(_, rest)| rest.split('/').next())
        .unwrap_or_default();
    if authority.is_empty() || authority.contains('*') {
        return Err("API server URL must include a concrete host".into());
    }
    Ok(value.trim_end_matches('/').to_string())
}

/// Centralized application configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    /// Human-readable application name.
    pub app_name: String,
    /// Unique bundle / package identifier (e.g. "dev.dioxus.starter").
    pub app_id: String,
    /// Application semantic version string.
    pub version: String,
    /// Server networking configuration.
    pub server: ServerConfig,
}

/// Server networking configuration and endpoint resolution.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
    /// Active resolved server URL.
    pub url: String,
    /// Default port for local backend server instances.
    pub default_port: u16,
    /// Localhost loopback endpoint.
    pub localhost_url: String,
    /// Default endpoint for the Android emulator.
    pub android_emulator_url: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            url: DEFAULT_LOCALHOST_URL.to_string(),
            default_port: DEFAULT_SERVER_PORT,
            localhost_url: DEFAULT_LOCALHOST_URL.to_string(),
            android_emulator_url: DEBUG_SERVER_URL.to_string(),
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_name: "Dioxus Cross-Platform Starter".to_string(),
            app_id: "dev.dioxus.starter".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            server: ServerConfig::default(),
        }
    }
}

impl AppConfig {
    /// Resolves the canonical server URL based on a strict 4-tier precedence:
    ///
    /// 1. **In-App Dynamic Override**: Value stored in `KeyValueStore` (e.g., via Settings).
    /// 2. **Process Runtime Environment**: `std::env::var("STARTER_SERVER_URL")` (Desktop/Server).
    /// 3. **Compile-Time Baked Environment**: `option_env!("STARTER_SERVER_URL")` (Baked into mobile/native binary).
    /// 4. **Platform Default Fallback**:
    ///    - Android (`PlatformTarget::Mobile(MobileOs::Android)`): `http://10.0.2.2:8080`
    ///    - All other targets: `http://127.0.0.1:8080`
    pub fn resolve_server_url(
        target: &PlatformTarget,
        storage_override: Option<String>,
        runtime_env: Option<String>,
        build_time_env: Option<&str>,
    ) -> String {
        // Tier 1: In-app persistent storage override
        if let Some(url) = storage_override {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        // Tier 2: Process runtime environment variable
        if let Some(url) = runtime_env {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        // Tier 3: Compile-time baked environment variable
        if let Some(url) = build_time_env {
            let trimmed = url.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }

        // Tier 4: Platform default fallback
        match target {
            PlatformTarget::Mobile(MobileOs::Android) => DEBUG_SERVER_URL.to_string(),
            _ => DEFAULT_LOCALHOST_URL.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DesktopOs;

    #[test]
    fn test_default_config() {
        let config = AppConfig::default();
        assert_eq!(config.app_id, "dev.dioxus.starter");
        assert_eq!(config.server.default_port, 8080);
        assert_eq!(config.server.localhost_url, "http://127.0.0.1:8080");
        assert_eq!(config.server.android_emulator_url, "http://10.0.2.2:8080");
    }

    #[test]
    fn validate_server_url_accepts_remote_and_same_origin_values() {
        assert_eq!(
            validate_server_url("https://api.example.com/").unwrap(),
            "https://api.example.com"
        );
        assert_eq!(validate_server_url("/api/").unwrap(), "/api");
        assert!(validate_server_url("ftp://api.example.com").is_err());
        assert!(validate_server_url("https://").is_err());
    }

    #[test]
    fn test_storage_override_precedence() {
        let target = PlatformTarget::Mobile(MobileOs::Android);
        let url = AppConfig::resolve_server_url(
            &target,
            Some("https://custom.api.com".to_string()),
            Some("https://env.api.com".to_string()),
            Some("https://build.api.com"),
        );
        assert_eq!(url, "https://custom.api.com");
    }

    #[test]
    fn test_runtime_env_precedence_over_build() {
        let target = PlatformTarget::Desktop(DesktopOs::Linux);
        let url = AppConfig::resolve_server_url(
            &target,
            None,
            Some("https://runtime.api.com".to_string()),
            Some("https://build.api.com"),
        );
        assert_eq!(url, "https://runtime.api.com");
    }

    #[test]
    fn test_build_env_precedence_over_fallback() {
        let target = PlatformTarget::Mobile(MobileOs::Android);
        let url = AppConfig::resolve_server_url(&target, None, None, Some("https://prod.api.com"));
        assert_eq!(url, "https://prod.api.com");
    }

    #[test]
    fn test_platform_default_android() {
        let target = PlatformTarget::Mobile(MobileOs::Android);
        let url = AppConfig::resolve_server_url(&target, None, None, None);
        assert_eq!(url, "http://10.0.2.2:8080");
    }

    #[test]
    fn test_platform_default_desktop_and_web() {
        let desktop = PlatformTarget::Desktop(DesktopOs::MacOS);
        assert_eq!(
            AppConfig::resolve_server_url(&desktop, None, None, None),
            "http://127.0.0.1:8080"
        );

        let web = PlatformTarget::Web;
        assert_eq!(
            AppConfig::resolve_server_url(&web, None, None, None),
            "http://127.0.0.1:8080"
        );
    }
}
