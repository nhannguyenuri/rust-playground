use crate::utils;
use axum::{extract::Request, http::StatusCode, response::IntoResponse};

pub async fn get_root(req: Request) -> impl IntoResponse {
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    let method = req.method().clone();
    let uri = req.uri().clone();
    let status = StatusCode::OK;

    utils::logger::info(&format!("[{}] [{}] [{}] [{}]", host, method, uri, status));

    (status, "Hello, World!")
}
