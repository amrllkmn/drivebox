use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{query_as, FromRow, Pool, Postgres};

#[derive(Debug, Deserialize, Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    verified: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "OK"})))
}

// an endpoint to
pub async fn register() {
    todo!()
}

pub async fn get_users(State(pool): State<Pool<Postgres>>) -> (StatusCode, Json<Value>) {
    let results = query_as::<_, User>("SELECT * FROM users ORDER BY updated_at DESC")
        .fetch_all(&pool)
        .await;

    match results {
        Ok(users) => (StatusCode::OK, Json(json! {users})),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong"})),
        ),
    }
}
