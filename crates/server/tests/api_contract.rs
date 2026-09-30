//! Contract tests for the API-only server.
//!
//! These mount the exact router the `server` binary serves, so they catch drift
//! between what a client fetches and what the server actually exposes. The
//! browser adapter in `api` and the server function in `api` are separate code
//! paths that must agree on the same path, payload shape, and CORS behavior.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::{Duration, Instant};

const READ_TIMEOUT: Duration = Duration::from_secs(5);

/// Start the router on an ephemeral port and return the bound address.
async fn spawn_server() -> String {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test server binds an ephemeral port");
    let addr = listener.local_addr().expect("listener has a local address");

    tokio::spawn(async move {
        let _ = axum::serve(listener, server::router()).await;
    });

    format!("{addr}")
}

/// True once `raw` holds a full head plus the advertised body.
fn is_complete(raw: &[u8]) -> bool {
    let text = String::from_utf8_lossy(raw);
    let Some((head, body)) = text.split_once("\r\n\r\n") else {
        return false;
    };
    match header_of(head, "content-length").and_then(|value| value.parse::<usize>().ok()) {
        Some(length) => body.len() >= length,
        // No length to honour: a complete head is all we can verify, and the
        // read timeout below bounds the wait.
        None => !head.is_empty(),
    }
}

fn header_of(head: &str, name: &str) -> Option<String> {
    head.lines()
        .find(|line| {
            line.to_ascii_lowercase()
                .starts_with(&name.to_ascii_lowercase())
        })
        .and_then(|line| line.split_once(':'))
        .map(|(_, value)| value.trim().to_string())
}

/// Issue a GET and return the raw response.
///
/// Reads against `Content-Length` with a deadline rather than waiting for the
/// peer to hang up, because the server keeps the connection alive.
fn get(addr: &str, path: &str) -> String {
    let mut stream = TcpStream::connect(addr).expect("test server accepts connections");
    stream
        .set_read_timeout(Some(READ_TIMEOUT))
        .expect("read timeout is settable");

    let request = format!(
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nOrigin: http://127.0.0.1:1\r\nConnection: close\r\n\r\n"
    );
    stream
        .write_all(request.as_bytes())
        .expect("request is written");
    stream.flush().expect("request is flushed");

    let deadline = Instant::now() + READ_TIMEOUT;
    let mut raw: Vec<u8> = Vec::new();
    let mut chunk = [0u8; 1024];

    loop {
        if is_complete(&raw) {
            break;
        }
        match stream.read(&mut chunk) {
            Ok(0) => break,
            Ok(read) => raw.extend_from_slice(&chunk[..read]),
            Err(_) => break,
        }
        if Instant::now() >= deadline {
            break;
        }
    }

    String::from_utf8_lossy(&raw).into_owned()
}

fn header(response: &str, name: &str) -> Option<String> {
    let head = response.split_once("\r\n\r\n")?.0;
    header_of(head, name)
}

fn body(response: &str) -> &str {
    response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .unwrap_or_default()
}

fn status_line(response: &str) -> &str {
    response.lines().next().unwrap_or_default()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn server_function_serves_the_status_contract() {
    let addr = spawn_server().await;
    let response = get(&addr, api::STATUS_PATH);

    assert_eq!(status_line(&response), "HTTP/1.1 200 OK");
    // The browser adapter decodes straight into this type, so the body must be
    // the exact payload the server function serializes.
    assert_eq!(
        body(&response).trim(),
        r#"{"status":"ok","service":"server"}"#
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn status_route_matches_the_constant_clients_request() {
    let addr = spawn_server().await;
    // Guards the one thing the compiler cannot: the `#[get]` attribute needs a
    // literal, so `api::STATUS_PATH` is a separate copy of the same route.
    assert_eq!(
        status_line(&get(&addr, api::STATUS_PATH)),
        "HTTP/1.1 200 OK",
        "the registered server function does not answer at api::STATUS_PATH"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn health_probe_matches_the_server_function_payload() {
    let addr = spawn_server().await;
    let response = get(&addr, "/api/v1/health");

    assert_eq!(status_line(&response), "HTTP/1.1 200 OK");
    assert_eq!(
        body(&response).trim(),
        body(&get(&addr, api::STATUS_PATH)).trim(),
        "the probe route drifted from the server function"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cross_origin_reads_are_allowed() {
    let addr = spawn_server().await;
    // The web client runs on its own dev port, so without this header the
    // browser discards the response even though the status is 200.
    for path in [api::STATUS_PATH, "/api/v1/health"] {
        let response = get(&addr, path);
        assert_eq!(
            header(&response, "access-control-allow-origin").as_deref(),
            Some("*"),
            "{path} is unreadable from the standalone web client"
        );
    }
}
