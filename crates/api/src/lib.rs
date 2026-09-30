use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct ServerStatus {
    pub status: String,
    pub service: String,
}

#[cfg(feature = "fullstack")]
use dioxus::prelude::*;

#[cfg(feature = "fullstack")]
#[get("/api/v1/status")]
pub async fn get_server_status() -> Result<ServerStatus, ServerFnError> {
    #[cfg(feature = "server")]
    {
        Ok(ServerStatus {
            status: "ok".to_string(),
            service: "server".to_string(),
        })
    }

    #[cfg(not(feature = "server"))]
    {
        unreachable!("server function client stub is executed remotely")
    }
}
