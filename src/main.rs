//! Chatrs entry point.

use chatrs::{app, db};
use std::error::Error;
use tokio::net::TcpListener;

/// Starts the http server.
/// Does this by creating the db, router, and listener.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = db::new().await?;
    let router = app::new(pool);
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, router).await?;
    Ok(())
}
