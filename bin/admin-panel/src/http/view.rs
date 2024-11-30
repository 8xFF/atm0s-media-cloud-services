use poem::Route;

/// only include in release build
#[cfg(not(debug_assertions))]
#[derive(rust_embed::Embed)]
#[folder = "view/dist"]
pub struct ViewFiles;

pub fn build_frontend_route() -> Route {
    #[cfg(debug_assertions)]
    {
        use poem::EndpointExt;
        let pconfig = http_common::dev_proxy::ProxyConfig::new("localhost:5173")
            .web_insecure() // Enables proxy-ing web requests, sets the proxy to use http instead of https
            .enable_nesting() // Sets the proxy to support nested routes
            .finish(); // Finishes constructing the configuration

        log::info!("Running in development mode, starting Vite dev server...");
        #[allow(unknown_lints)]
        #[allow(clippy::zombie_processes)]
        std::process::Command::new("pnpm")
            .current_dir("./view/")
            .args(["run", "dev"])
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .expect("Failed to start Vite dev server");

        // Proxy frontend requests to Vite
        Route::new().nest("/", http_common::dev_proxy::proxy.data(pconfig))
    }

    #[cfg(not(debug_assertions))]
    {
        Route::new()
            .at(
                "/",
                http_common::emdedded_files::EmbeddedFileEndpoint::<ViewFiles>::new("index.html"),
            )
            .nest(
                "/",
                http_common::emdedded_files::EmbeddedFilesEndpoint::<ViewFiles>::new(),
            )
    }
}
