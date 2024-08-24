use crate::{user::User, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    Json,
};
use oauth2::{reqwest::async_http_client, AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};

async fn handle_user(pool: Pool<Postgres>, user_info: UserInfo) -> (StatusCode, Json<Value>) {
    let result = User::create(&pool, user_info).await;

    match result {
        Ok(user) => (StatusCode::OK, Json(json!(user))),
        Err(err) => {
            println!("{:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"message":"Something went wrong"})),
            )
        }
    }
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
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    picture: String,
}

pub async fn healthcheck() -> (StatusCode, Json<Value>) {
    (StatusCode::OK, Json(json!({"message": "OK"})))
}

// an endpoint to
pub async fn register(State(app_state): State<AppState>) -> impl IntoResponse {
    // 1. form the parameters
    // 2. redirect to the auth server
    let scopes = [
        Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
        Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
    ];

    let (auth_url, _) = app_state
        .client
        .authorize_url(CsrfToken::new_random)
        .add_scopes(scopes)
        .url();

    Redirect::to(auth_url.as_str())
}

pub async fn callback(
    Query(query): Query<AuthRequest>,
    State(app_state): State<AppState>,
) -> (StatusCode, Json<Value>) {
    // get access token
    let token = app_state
        .client
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

                handle_user(app_state.database, user_info).await
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

pub async fn get_users(State(app_state): State<AppState>) -> (StatusCode, Json<Value>) {
    let users = User::get_all(&app_state.database).await;
    match users {
        Ok(users) => (StatusCode::OK, Json(json! {users})),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"message": "Something went wrong"})),
        ),
    }
}
