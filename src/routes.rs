use crate::handlers;
use axum::{routing::get, Router};
use sqlx::{Pool, Postgres};

pub fn create_api_route(app_state: Pool<Postgres>) -> Router {
    let api_routes = Router::new().route("/users", get(handlers::get_users));
    Router::new()
        .route("/healthcheck", get(handlers::healthcheck))
        .nest("/api/v1", api_routes)
        .with_state(app_state)
}
