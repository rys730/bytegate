use std::sync::Arc;

use axum::{routing::post, Json, Router};

use crate::app::{common::config::ServiceConfig, infrastructure::postgres::postgres::DB, repository::user::UserRepository, usecase::user::UserUsecase};

pub fn new_user_routes(db: Arc<DB>, cfg: Arc<ServiceConfig>) -> Router {
    let repo = Arc::new(UserRepository::new(db));
    let usecase = Arc::new(UserUsecase::new(cfg, repo));
    Router::new()
    .route("/create", post(Json("hello")))
    .with_state(usecase)
}