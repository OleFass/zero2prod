use axum::{
    Router, Json,
    routing::{get, post},
    response::{Html, IntoResponse},
    http::StatusCode
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    // create a TcpListener and bind it to 127.0.0.1:3000
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server listening on: {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

/* 
equivalent to declaring in main():
let app = Router::new().route("/", get(hello));
*/
fn app() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/post_json", post(print_req))
}

/* 
basic handler that responds with static HTML
curl -X GET http://localhost:3000/hello
*/ 
async fn hello() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

/*
curl -X POST -H "Content-Type: application/json" -d '{"some_string":"loremipsum"}' http://localhost:3000/post_json
*/
async fn print_req(
    Json(payload): Json<ReqBody>,
) -> impl IntoResponse {
    let res_body = ResBody {
        id: 1337,
        some_string: payload.some_string,
    };
    // <status code of `201 Created`, converted into a JSON response>
    (StatusCode::CREATED, Json(res_body))
}

/*
async fn health_check(req: ) {

}
*/

/*
async fn greet() {

}
*/

// the input to our `print_req` handler
#[derive(Deserialize)]
struct ReqBody {
    some_string: String,
}

// the output to our `print_req` handler
#[derive(Serialize)]
struct ResBody {
    id: u64,
    some_string: String,
}