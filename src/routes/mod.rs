use axum::Router;
use axum::routing::get;
use crate::handlers::health_check::health_check;

pub fn router_entry_point() -> Router {
    Router::new()
        .route("/health", get(health_check))
}