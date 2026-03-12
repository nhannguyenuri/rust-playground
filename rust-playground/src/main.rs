mod config;
mod routes;
mod utils;

use axum::{Router, routing::get};
use dotenvy::dotenv;
use std::env;
use tracing::info;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let env = env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string());
    let host_name = env::var("HOST_NAME").unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    info!("Starting server in {env} environment");

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(routes::v1::root::get_root))
        .route("/api/v1/ping", get(routes::v1::ping::get_ping));

    let listener = tokio::net::TcpListener::bind(&format!("{host_name}:{port}"))
        .await
        .unwrap();

    info!("Server is running at {host_name}:{port}");

    axum::serve(listener, app).await.unwrap();
}
