pub const DEFAULT_SERVER_API_URL: &str = "http://127.0.0.1:8080";

pub fn server_api_url() -> &'static str {
    option_env!("SERVER_API_URL")
        .filter(|url| !url.trim().is_empty())
        .or_else(|| option_env!("DEV_SERVER_API_URL").filter(|url| !url.trim().is_empty()))
        .unwrap_or(DEFAULT_SERVER_API_URL)
}
