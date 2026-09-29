use starter_core::ServerStatus;
use std::time::Instant;
use tower_http::cors::{AllowOrigin, CorsLayer};

/// Build the cross-origin policy for the fullstack server.
///
/// Origins are read from `CORS_ALLOWED_ORIGINS` as a comma-separated list.
/// An unset value keeps the server same-origin only. Wildcards are rejected
/// because this policy enables credentials for authenticated applications.
pub fn cors_layer_from_env() -> Result<CorsLayer, String> {
    let raw = std::env::var("CORS_ALLOWED_ORIGINS").unwrap_or_default();
    let origins = raw
        .split(',')
        .map(str::trim)
        .filter(|origin| !origin.is_empty())
        .map(parse_origin)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([http::Method::GET, http::Method::POST, http::Method::OPTIONS])
        .allow_headers([http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        .allow_credentials(true))
}

fn parse_origin(origin: &str) -> Result<http::HeaderValue, String> {
    if origin == "*" || origin.contains('*') {
        return Err("CORS_ALLOWED_ORIGINS cannot contain '*' when credentials are enabled".into());
    }
    if !(origin.starts_with("http://") || origin.starts_with("https://"))
        || origin.chars().any(char::is_whitespace)
        || origin.ends_with('/')
    {
        return Err(format!(
            "invalid CORS origin '{origin}'; use an origin such as https://app.example.com"
        ));
    }
    http::HeaderValue::from_str(origin).map_err(|_| format!("invalid CORS origin '{origin}'"))
}

pub mod paystack;

static START_TIME: std::sync::OnceLock<Instant> = std::sync::OnceLock::new();

pub fn init_server_state() {
    START_TIME.get_or_init(Instant::now);
}

/// Retrieve server status.
pub async fn get_status() -> ServerStatus {
    init_server_state();
    let uptime = START_TIME.get().map(|t| t.elapsed().as_secs()).unwrap_or(0);

    ServerStatus {
        ok: true,
        uptime_seconds: uptime,
        message: "Dioxus Cross-Platform Fullstack Server operational".to_string(),
        environment: if cfg!(debug_assertions) {
            "development".to_string()
        } else {
            "production".to_string()
        },
    }
}

#[cfg(test)]
mod tests {
    use super::parse_origin;

    #[test]
    fn cors_rejects_wildcards_with_credentials() {
        assert!(parse_origin("*").is_err());
        assert!(parse_origin("https://*.example.com").is_err());
    }

    #[test]
    fn cors_requires_origins_without_paths_or_whitespace() {
        assert!(parse_origin("https://app.example.com/").is_err());
        assert!(parse_origin("https://app.example.com/path").is_ok());
        assert!(parse_origin("https://app example.com").is_err());
    }
}
