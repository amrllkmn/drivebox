use axum::{http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(healthcheck_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn healthcheck_handler() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "OK"})))
}
