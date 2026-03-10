mod utils;
mod routes;
mod config;

use axum::{
    Json, Router,
    extract::Request,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
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
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user));

    // run our app with hyper
    let listener = tokio::net::TcpListener::bind(&format!("{host_name}:{port}"))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    // print a message when the server is ready
    utils::logger::info(&format!("Server is running at {host_name}:{port}"));

    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root(req: Request) -> impl IntoResponse {
    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown");
    let method = req.method().clone();
    let uri = req.uri().clone();
    let status = StatusCode::OK;

    utils::logger::info(&format!(
        "[{}] [{}] [{}] [{}]",
        host, method, uri, status
    ));

    (status, "Hello, World!")
}

async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}

// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
