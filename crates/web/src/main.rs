#[cfg(feature = "server")]
use starter_ui::App;

fn main() {
    #[cfg(feature = "server")]
    {
        let _ = dotenvy::dotenv();
        let cors = match starter_server::cors_layer_from_env() {
            Ok(cors) => cors,
            Err(error) => {
                eprintln!("server configuration error: {error}");
                std::process::exit(2);
            }
        };
        dioxus::serve(move || {
            let cors = cors.clone();
            async move { Ok(dioxus::server::router(App).layer(cors)) }
        });
    }

    #[cfg(not(feature = "server"))]
    starter_web::launch_client();
}
