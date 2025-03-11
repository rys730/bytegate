use axum::{Router};
use crate::app::{common::config::AppConfig, handler::{healthcheck::new_healthcheck_routes, shortener::new_shortener_routes}, infrastructure::postgres::postgres::DB};

pub fn create_routes(db: DB) -> Router {
    let healthcheck_routes = new_healthcheck_routes();
    let shortener_routes = new_shortener_routes(db);
    let app = Router::new()
        .nest("/healthcheck", healthcheck_routes)
        .nest("/shortener", shortener_routes);
    app
}