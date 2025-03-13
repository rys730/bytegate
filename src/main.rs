mod routes;
mod app;

use std::sync::Arc;

use crate::routes::create_routes;
use crate::app::common::config;
use crate::app::infrastructure::postgres::postgres;

#[tokio::main]
async fn main() {
    let cfg = config::load_config();
    let db = postgres::DB::new(&cfg.db_config).await.expect("Failed connecting to DB");
    let app = create_routes(Arc::new(db), Arc::new(cfg));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server is running on http://127.0.0.1:3000");
    axum::serve(listener, app.into_make_service()).await.unwrap();
}
