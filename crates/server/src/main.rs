fn main() {
    dioxus::serve(|| async { Ok(server::router()) });
}
