use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope,
    TokenResponse,
};
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

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct AuthRequest {
    code: String,
    state: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(dead_code)]
pub struct UserInfo {
    email: String,
    verified_email: bool,
    name: String,
    given_name: String,
    picture: String,
}

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "OK"})))
}

// an endpoint to
pub async fn register(State(client): State<BasicClient>) -> impl IntoResponse {
    // 1. form the parameters
    // 2. redirect to the auth server
    let scopes = [
        Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
        Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
    ];

    let (auth_url, _) = client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes)
        .url();

    Redirect::to(auth_url.as_str())
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    State(client): State<BasicClient>,
) -> (StatusCode, Json<Value>) {
    // get access token
    let token = client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(async_http_client)
        .await
        .unwrap();

    let request_client = reqwest::Client::new();

    let user_data = request_client
        .get("https://www.googleapis.com/oauth2/v1/userinfo?alt=json")
        .bearer_auth(token.access_token().secret())
        .send()
        .await;

    match user_data {
        Ok(resp) => {
            if resp.status().is_success() {
                let user_info = resp
                    .json::<UserInfo>()
                    .await
                    .expect("Failed to deserialise data");

                (StatusCode::OK, Json(json!(user_info)))
            } else {
                (
                    resp.status(),
                    Json(json!({"message":"Something went wrong"})),
                )
            }
        }
        Err(err) => {
            println!("{:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message":"Internal server error"})),
            )
        }
    }
    // store user data
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
