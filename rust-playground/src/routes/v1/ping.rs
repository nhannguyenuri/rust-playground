use crate::utils;
use axum::{Json, extract::Request, http::StatusCode, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct PingResponse {
    sucess: bool,
    data: String,
}

pub async fn get_ping(req: Request) -> impl IntoResponse {
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    let method = req.method().clone();
    let uri = req.uri().clone();
    let status = StatusCode::OK;

    let ping_response = PingResponse {
        sucess: true,
        data: "pong".to_string(),
    };

    utils::logger::info(&format!("[{}] [{}] [{}] [{}]", host, method, uri, status));

    (StatusCode::OK, Json(ping_response))
}
