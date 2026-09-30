pub const DEFAULT_SERVER_API_URL: &str = "http://127.0.0.1:8080";

/// Compile-time baked API base URL, or `None` when nothing was configured.
///
/// A browser SPA must be able to tell "not configured" apart from "configured
/// to the native fallback", because its correct default is same-origin rather
/// than an absolute loopback address.
pub fn server_api_url_override() -> Option<&'static str> {
    option_env!("SERVER_API_URL")
        .filter(|url| !url.trim().is_empty())
        .or_else(|| option_env!("DEV_SERVER_API_URL").filter(|url| !url.trim().is_empty()))
}

/// Resolved API base URL for native clients, which always need an absolute target.
pub fn server_api_url() -> &'static str {
    server_api_url_override().unwrap_or(DEFAULT_SERVER_API_URL)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn override_presence_drives_the_fallback() {
        match server_api_url_override() {
            Some(url) => assert_eq!(server_api_url(), url),
            None => assert_eq!(server_api_url(), DEFAULT_SERVER_API_URL),
        }
    }

    #[test]
    fn resolved_url_is_never_blank() {
        assert!(!server_api_url().trim().is_empty());
        assert!(!server_api_url().contains(char::is_whitespace));
    }

    #[test]
    fn native_fallback_targets_loopback_over_http() {
        assert!(DEFAULT_SERVER_API_URL.starts_with("http://"));
    }
}
