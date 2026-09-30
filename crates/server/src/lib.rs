use api::ServerStatus;

/// The single health payload served by `/api/v1/health` and `/api/v1/status`.
pub fn health_payload() -> ServerStatus {
    ServerStatus {
        status: "ok".to_string(),
        service: "utme-lab-server".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_payload_reports_ok() {
        let payload = health_payload();
        assert_eq!(payload.status, "ok");
        assert_eq!(payload.service, "utme-lab-server");
    }

    #[test]
    fn health_payload_matches_wire_contract() {
        let json = serde_json::to_string(&health_payload()).expect("health payload serializes");
        assert_eq!(json, r#"{"status":"ok","service":"utme-lab-server"}"#);
    }
}
