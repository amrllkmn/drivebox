mod handlers;
mod routes;
mod user;

use axum::extract::FromRef;
use dotenv::dotenv;
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    database: Pool<Postgres>,
    client: BasicClient,
}

impl FromRef<AppState> for Pool<Postgres> {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

impl FromRef<AppState> for BasicClient {
    fn from_ref(state: &AppState) -> Self {
        state.client.clone()
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    let addr = "0.0.0.0:8080"; // todo: change me to run without docker
    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL env");
    let auth_url = env::var("GOOGLE_AUTH_URI").expect("Missing GOOGLE_AUTH_URI");
    let token_url = env::var("GOOGLE_TOKEN_URI").expect("Missing GOOGLE_TOKEN_URI");
    let client_id = env::var("GOOGLE_CLIENT_ID").expect("Missing GOOGLE_CLIENT_ID");
    let client_secret = env::var("GOOGLE_CLIENT_SECRET").expect("Missing GOOGLE_CLIENT_SECRET");

    let oauth_client = oauth_client(auth_url, token_url, client_id, client_secret);

    let app_state: AppState = AppState {
        database: PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap(),
        client: oauth_client,
    };

    sqlx::migrate!()
        .run(&app_state.database)
        .await
        .expect("Migrating went wrong");
    let app = routes::create_api_route(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn oauth_client(
    auth_url: String,
    token_url: String,
    client_id: String,
    client_secret: String,
) -> BasicClient {
    dotenv().ok();
    let auth_url = AuthUrl::new(auth_url).expect("Error parsing the auth url");
    let token_url = TokenUrl::new(token_url).expect("Error parsing the token url");

    let client_id = ClientId::new(client_id);
    let client_secret = ClientSecret::new(client_secret);

    let redirect_url = "http://localhost:8080/auth/callback".to_string();

    BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
        .set_redirect_uri(RedirectUrl::new(redirect_url).expect("Error parsing the redirect_url"))
}
