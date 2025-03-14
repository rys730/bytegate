use std::sync::Arc;

use axum::Router;
use crate::app::{common::config::AppConfig, handler::{healthcheck::new_healthcheck_routes, shortener::new_shortener_routes}, infrastructure::postgres::postgres::DB};

pub fn create_routes(db: Arc<DB>, cfg: Arc<AppConfig>) -> Router {
    let healthcheck_routes = new_healthcheck_routes();
    let config = Arc::new(cfg);
    let service_config = Arc::new(config.service_config.clone());
    let shortener_routes = new_shortener_routes(db.clone(), service_config.clone());

    let app = Router::new()
        .merge(healthcheck_routes)
        .merge(shortener_routes);
    app
}