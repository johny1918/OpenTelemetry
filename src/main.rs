use dotenvy::dotenv;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod error;
mod handlers;
mod models;
mod routes;
mod state;

use routes::router_entry_point;

use tokio::net::TcpListener;
use crate::models::user::User;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Load env vars
    dotenv().ok();

    let db_pool = Arc::new(Mutex::new(HashMap::<i32, User>::new()));

    //Start server
    let app = router_entry_point();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
