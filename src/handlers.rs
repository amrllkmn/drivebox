use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "OK"})))
}
