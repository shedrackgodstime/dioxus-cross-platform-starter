#[cfg(feature = "server")]
use starter_ui::App;

fn main() {
    #[cfg(feature = "server")]
    {
        let _ = dotenvy::dotenv();
        dioxus::serve(|| async move { Ok(dioxus::server::router(App)) });
    }

    #[cfg(not(feature = "server"))]
    starter_web::launch_client();
}
