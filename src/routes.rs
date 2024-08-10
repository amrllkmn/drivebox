use crate::handlers;
use axum::{routing::get, Router};

pub fn create_api_route() -> Router {
    Router::new().route("/healthcheck", get(handlers::healthcheck))
}
