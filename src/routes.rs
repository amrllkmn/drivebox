use crate::{handlers, AppState};
use axum::{routing::get, Router};

pub fn create_api_route(app_state: AppState) -> Router {
    let api_routes = Router::new().route("/users", get(handlers::get_users));
    let auth_routes = Router::new()
        .route("/register", get(handlers::register))
        .route("/callback", get(handlers::callback));
    Router::new()
        .route("/healthcheck", get(handlers::healthcheck))
        .nest("/api/v1", api_routes)
        .nest("/auth", auth_routes)
        .with_state(app_state)
}
