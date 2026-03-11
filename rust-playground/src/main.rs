mod config;
mod routes;
mod utils;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().ok();
    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    let host_name = env::var("HOST_NAME").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    utils::logger::info(&format!("Starting server in {env} environment"));

    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(routes::v1::root::get_root))
        .route("/api/v1/ping", get(routes::v1::ping::get_ping));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(&format!("{host_name}:{port}"))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // print a message when the server is ready
    utils::logger::info(&format!("Server is running at {host_name}:{port}"));

    // serve the app
    axum::serve(listener, app).await.unwrap();
}
