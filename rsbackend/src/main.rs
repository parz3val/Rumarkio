use router::web_router;
pub mod handlers;
pub mod helpers;
pub mod router;
pub mod store;
pub mod types;
use axum::{self, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // Start the tracer
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    // let app = Router::new();
    let app: Router = web_router().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
