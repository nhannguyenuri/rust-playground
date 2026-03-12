use axum::{extract::Request, http::StatusCode, response::IntoResponse};
use tracing::info;

pub async fn get_root(req: Request) -> impl IntoResponse {
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    let method = req.method().clone();
    let uri = req.uri().clone();
    let status = StatusCode::OK;

    info!("[{}] [{}] [{}] [{}]", host, method, uri, status);

    (status, "Hello, World!")
}
