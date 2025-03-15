use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use hyper::StatusCode;

use crate::app::{common::errors::ServiceError, infrastructure::postgres::postgres::DB, model::dto::user::RegisterUserRequest, repository::user::UserRepository, usecase::user::UserUsecase};


pub async fn register_new_user(
    State(usecase): State<Arc<UserUsecase>>,
    Json(request): Json<RegisterUserRequest>,
) -> Result<impl IntoResponse, ServiceError>{
    let res= usecase.register_user(request).await;
    match res {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(e) => Err(e),
    }
}

pub fn new_user_routes(db: Arc<DB>) -> Router {
    let repo = Arc::new(UserRepository::new(db));
    let usecase = Arc::new(UserUsecase::new(repo));
    Router::new()
    .route("/register", post(register_new_user))
    .with_state(usecase)
}