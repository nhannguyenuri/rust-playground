use axum::{Json, extract::Request, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use tracing::info;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct PingResponse {
    success: bool,
    data: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/ping",
    tag = "ping",
    responses(
        (status = 200, description = "Get ping successfully", body = PingResponse),
    )
)]
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
        success: true,
        data: "pong".to_string(),
    };

    info!("[{}] [{}] [{}] [{}]", host, method, uri, status);

    (StatusCode::OK, Json(ping_response))
}
