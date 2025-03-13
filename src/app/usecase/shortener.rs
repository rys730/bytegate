use std::sync::Arc;

use crate::app::{
    common::{
        config::ServiceConfig, utils, errors::Result
    }, 
    model::dto::{
        shortener::ShortenerResponse, url_maps::UrlMapDB
    }, 
    repository::shortener::{
        ShortenerRepository, ShortenerRepositoryTrait
    }
};

pub struct ShortenerUsecase {
    cfg: Arc<ServiceConfig>,
    shortener_repository: Arc<dyn ShortenerRepositoryTrait>,
}

impl  ShortenerUsecase {
    pub fn new(cfg: Arc<ServiceConfig>, repo: Arc<ShortenerRepository>) -> Self {
        Self { 
            shortener_repository: repo,
            cfg: cfg 
        }
    }
}

impl ShortenerUsecase  {
    pub async fn create_short_url(&self, url: String) -> Result<ShortenerResponse> {
        let short_code = utils::hash_url(&url);
        let short_url = self.cfg.base_url.clone() + &short_code;
        let short_dto = self.shortener_repository.create_url_mapping(UrlMapDB{
            destination_url: url,
            short_url: short_url,
            ..Default::default()
        }).await?;
        Ok(ShortenerResponse{short_url: short_dto.short_url})
    }
}

