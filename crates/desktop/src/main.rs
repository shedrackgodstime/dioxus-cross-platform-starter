fn main() {
    dioxus::fullstack::set_server_url(core::config::server_api_url());
    dioxus::launch(ui::App);
}
