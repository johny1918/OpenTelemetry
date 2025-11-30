use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::handlers::health_check::health_check;
use axum::Router;
use axum::routing::{get, post};
use crate::handlers::database_operations::create_user;
use crate::models::user::User;

pub fn router_entry_point(route: Arc<Mutex<HashMap<i32, User>>>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user))
        .with_state(route)
}
