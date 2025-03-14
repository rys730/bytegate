use std::sync::Arc;

use axum::extract::Path;
use axum::response::{IntoResponse, Redirect};
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::State, Json};
use axum::http::StatusCode;

use crate::app::common::config::ServiceConfig;
use crate::app::common::errors::ServiceError;
use crate::app::infrastructure::postgres::postgres::DB;
use crate::app::model::dto::shortener::{ShortenerRequest, ShortenerResponse};
use crate::app::repository::shortener::ShortenerRepository;
use crate::app::usecase::shortener::ShortenerUsecase;

pub async fn create_short_url(
    State(usecase): State<Arc<ShortenerUsecase>>,
    Json(request): Json<ShortenerRequest>,
) -> Result<impl IntoResponse, ServiceError> {
    let res = usecase.create_short_url(request.url).await;
    match res {
        Ok(response) => Ok((StatusCode::CREATED, Json(response))),
        Err(e) => Err(e),
    }
}

pub async fn get_url(
    State(usecase): State<Arc<ShortenerUsecase>>,
    Path(code): Path<String>,
) -> Result<impl IntoResponse, ServiceError>{
    match usecase.redirect_url(code).await {
        Ok(url) => Ok(Redirect::permanent(url.original_url.as_str())),
        Err(e) => Err(e)
    }
}

pub fn new_shortener_routes(db: Arc<DB>, cfg: Arc<ServiceConfig>) -> Router {
    let repo = Arc::new(ShortenerRepository::new(db));
    let usecase = Arc::new(ShortenerUsecase::new(cfg, repo));
    Router::new()
    .route("/create", post(create_short_url))
    .route("/{code}", get(get_url))
    .with_state(usecase)
}
