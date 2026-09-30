pub const DEFAULT_SERVER_API_URL: &str = "http://127.0.0.1:8080";

pub fn server_api_url() -> &'static str {
    option_env!("SERVER_API_URL").unwrap_or(DEFAULT_SERVER_API_URL)
}
