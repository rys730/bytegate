use std::sync::Arc;

use axum::response::IntoResponse;
use axum::routing::post;
use axum::Router;
use axum::{extract::State, Json};
use axum::http::StatusCode;

use crate::app::infrastructure::postgres::postgres::DB;
use crate::app::model::dto::shortener::{ShortenerRequest, ShortenerResponse};
use crate::app::repository::shortener::ShortenerRepository;
use crate::app::usecase::shortener::ShortenerUsecase;
use crate::app::usecase::shortener::ShortenerUsecaseTrait;

pub async fn create_short_url(
    State(usecase): State<Arc<dyn ShortenerUsecaseTrait>>,
    Json(request): Json<ShortenerRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let short_url = usecase.create_short_url(request.url);
    let response = ShortenerResponse { short_url };
    let res = Json(response);
    Ok((StatusCode::CREATED, res))
}

pub fn new_shortener_routes(db: DB) -> Router {
    let repo = Arc::new(ShortenerRepository::new(db));
    let usecase: Arc<dyn ShortenerUsecaseTrait> = Arc::new(ShortenerUsecase::new(repo));
    Router::new()
    .route("/create", post(create_short_url))
    .with_state(usecase)
}
