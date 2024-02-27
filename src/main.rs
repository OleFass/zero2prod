use axum::{
    http::StatusCode, 
    response::{
        Json,
        IntoResponse,
        Response
    }, 
    routing::get, 
    Router
};
use std::net::SocketAddr;
use chrono::Utc;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ping", get(ping_handler))
        .route("/return_json", get(return_json))
        .route("/health_check", get(health_check));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    println!("Server listening on {}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

/* */
async fn ping_handler() -> Response {
    let current_datetime = Utc::now().to_rfc3339();
    format!("Current datetime: {}", current_datetime).into_response()
}

/* */
async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(Serialize)]
struct Time {
    current_datetime: String,
}
async fn return_json() -> Response {
    let time = Time {
        current_datetime: Utc::now().to_rfc3339()
    };
    Json(time).into_response()
}