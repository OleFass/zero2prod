use axum::{
    routing::get,
    Router,
    response::{
        //Json,
        IntoResponse,
        Response
    },
};
use std::net::SocketAddr;
//use std::net::TcpListener;
use chrono::Utc; // For handling datetime
//use serde_json::json; // For creating JSON objects

#[tokio::main]
async fn main() {
    // build our application with a single route
    //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let app = Router::new()
        .route("/ping", get(ping_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    println!("Server listening on {}", addr);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// Handler for the `/ping` route, now returning JSON
/*
async fn ping_handler() -> Json<serde_json::Value> {
    let current_datetime = Utc::now().to_rfc3339();
    Json(json!({"current_datetime": current_datetime}))
}
 */
async fn ping_handler() -> Response {
    //let current_datetime = Utc::now().to_rfc3339();
    format!("Current datetime: {}", Utc::now().to_rfc3339()).into_response()
}