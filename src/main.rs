#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
  use dotenv::dotenv;

  use axum::{
    routing::{get, post},
    Router,
  };
  use fm_faucet::{component::App, fallback};
  use leptos::*;
  use leptos_axum::{generate_route_list, LeptosRoutes};
  use log::Level;

  // Load variables from .env file
  dotenv().ok();

  let conf = get_configuration(None).await.unwrap();
  let leptos_options = conf.leptos_options;
  let addr = leptos_options.site_addr;
  // Generate the list of routes in your Leptos App
  let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

  simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");
  let _ = console_log::init_with_level(Level::Debug);

  // build our application with a route
  let app = Router::new()
    .route("/favicon.ico", get(fallback::file_and_error_handler))
    .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
    .fallback(fallback::file_and_error_handler)
    .leptos_routes(&leptos_options, routes, |cx| view! { cx, <App/> })
    .with_state(leptos_options);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  log!("listening on {}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
