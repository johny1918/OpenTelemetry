use dotenvy::dotenv;

mod routes;
mod state;
mod handlers;
mod error;
mod models;

use tokio::net::TcpListener;
use routes::router_entry_point;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //Load env vars
    dotenv().ok();

    let app = router_entry_point();
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
