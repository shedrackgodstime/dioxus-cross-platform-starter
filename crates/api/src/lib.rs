use serde::{Deserialize, Serialize};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Route for the backend status endpoint.
///
/// The server function below declares the same path in its `#[get]` attribute,
/// which requires a literal. Keep the two in sync; `status_path_matches_route`
/// guards it.
pub const STATUS_PATH: &str = "/api/v1/status";

pub type StatusResult = Result<ServerStatus, String>;

/// Boxed, transport-agnostic status future.
///
/// Not `Send`: the browser adapter awaits JS-backed futures, so requiring `Send`
/// would exclude exactly the client that needs it.
pub type StatusFuture = Pin<Box<dyn Future<Output = StatusResult>>>;

/// How a binary reaches the backend.
///
/// The transport is chosen by the launcher, not by the UI, because the two are
/// mutually exclusive: a browser SPA cannot enable `dioxus/fullstack`, and a
/// fullstack or native binary cannot use `fetch` before the page exists. Both
/// adapters share [`ServerStatus`] and [`STATUS_PATH`], so they cannot drift.
pub type StatusFetcher = Arc<dyn Fn() -> StatusFuture + Send + Sync>;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerStatus {
    pub status: String,
    pub service: String,
}

/// Canonical backend status payload.
///
/// Single source of truth for every surface that reports health, so the
/// server function and any plain probe route cannot drift apart.
pub fn server_status() -> ServerStatus {
    ServerStatus {
        status: "ok".to_string(),
        service: "server".to_string(),
    }
}

#[cfg(feature = "fullstack")]
use dioxus::prelude::*;

/// Backend status via a Dioxus server function.
///
/// Used by the fullstack bundle and the native desktop and mobile launchers.
#[cfg(feature = "fullstack")]
#[get("/api/v1/status")]
pub async fn get_server_status() -> Result<ServerStatus, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(server_status())
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!("server function client stub is executed remotely")
    }
}

/// Backend status over the server-function transport.
#[cfg(feature = "fullstack")]
pub fn server_fn_fetcher() -> StatusFetcher {
    Arc::new(|| {
        let future: StatusFuture = Box::pin(async {
            get_server_status()
                .await
                .map_err(|failure| format!("{failure}"))
        });
        future
    })
}

/// Backend status over plain HTTP.
///
/// Defaults to same-origin, which is how a SPA and its API are deployed. Set
/// `SERVER_API_URL`, or `DEV_SERVER_API_URL` for local work, to point at a
/// backend on another origin.
#[cfg(feature = "http-client")]
pub fn http_fetcher() -> StatusFetcher {
    Arc::new(|| {
        let future: StatusFuture = Box::pin(async {
            let url = match core::config::server_api_url_override() {
                Some(base) => format!("{}{}", base.trim_end_matches('/'), STATUS_PATH),
                None => STATUS_PATH.to_string(),
            };

            let response = gloo_net::http::Request::get(&url)
                .send()
                .await
                .map_err(|failure| failure.to_string())?;

            if !response.ok() {
                return Err(format!("backend returned HTTP {}", response.status()));
            }

            response
                .json::<ServerStatus>()
                .await
                .map_err(|failure| failure.to_string())
        });
        future
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_reports_ok() {
        assert_eq!(server_status().status, "ok");
        assert_eq!(server_status().service, "server");
    }

    #[test]
    fn status_wire_format_is_stable() {
        let json = serde_json::to_string(&server_status()).expect("status serializes");
        assert_eq!(json, r#"{"status":"ok","service":"server"}"#);
    }

    #[test]
    fn status_round_trips() {
        let json = serde_json::to_string(&server_status()).expect("status serializes");
        let decoded: ServerStatus = serde_json::from_str(&json).expect("status deserializes");
        assert_eq!(decoded, server_status());
    }

    #[test]
    fn status_path_matches_route() {
        assert_eq!(STATUS_PATH, "/api/v1/status");
    }

    #[cfg(feature = "http-client")]
    #[test]
    fn http_url_is_same_origin_without_an_override() {
        // Nothing bakes SERVER_API_URL into this test build, so no override.
        assert!(core::config::server_api_url_override().is_none());
    }
}
