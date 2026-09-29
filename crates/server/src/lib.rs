use starter_core::ServerStatus;
use std::time::Instant;

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
