use axum::{routing::get, Router};
use bytegate::routes::{self, create_routes};

#[tokio::main]
async fn main() {
    let app = create_routes();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
