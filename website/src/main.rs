#[tokio::main]
#[cfg(feature = "ssr")]
async fn main() {
    pub fn shell(options: LeptosOptions) -> impl IntoView {
        view! {
            <!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="utf-8"/>
                    <meta name="viewport" content="width=device-width, initial-scale=1"/>
                    <AutoReload options=options.clone() />
                    <HydrationScripts options/>
                    <link rel="stylesheet" id="leptos" href="/pkg/esp-lsd-hacking.css"/>
                    <link rel="shortcut icon" type="image/ico" href="/favicon.ico"/>
                    <MetaTags/>
                </head>
                <body>
                    <App/>
                </body>
            </html>
        }
    }
    use axum::Router;
    use esp_lsd_hacking::app::*;
    use leptos::prelude::*;
    use leptos_meta::*;
    //use esp_lsd_hacking::fileserv::file_and_error_handler;
    use leptos::prelude::get_configuration;
    use leptos::*;

    use leptos_axum::{file_and_error_handler, generate_route_list, LeptosRoutes};

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler(shell))
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
