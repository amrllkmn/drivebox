use crate::handlers;
use axum::{routing::get, Router};
use oauth2::basic::BasicClient;
use sqlx::{Pool, Postgres};

pub fn create_api_route(database: Pool<Postgres>, oauth_client: BasicClient) -> Router {
    let api_routes = Router::new()
        .route("/users", get(handlers::get_users))
        .with_state(database);
    let auth_routes = Router::new()
        .route("/register", get(handlers::register))
        .route("/callback", get(handlers::callback))
        .with_state(oauth_client);
    Router::new()
        .route("/healthcheck", get(handlers::healthcheck))
        .nest("/api/v1", api_routes)
        .nest("/auth", auth_routes)
}
