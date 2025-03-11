use axum::{Router};
use crate::app::handler::{healthcheck::new_healthcheck_routes, shortener::new_shortener_routes};

pub fn create_routes() -> Router {
    let healthcheck_routes = new_healthcheck_routes();
    let shortener_routes = new_shortener_routes();
    let app = Router::new()
        .nest("/healthcheck", healthcheck_routes)
        .nest("/shortener", shortener_routes);
    app
}