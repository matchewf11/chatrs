use sqlx::SqlitePool;
use std::error::Error;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // connect_with for options
    let _pool = SqlitePool::connect("sqlite::memory:").await?;

    let router = chatrs::create_app();

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, router).await?;

    Ok(())
}
