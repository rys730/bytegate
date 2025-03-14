use std::sync::Arc;

use crate::app::{
    common::{
        config::ServiceConfig, errors::Result, utils
    }, 
    model::dto::{
        shortener::{RedirectResponse, ShortenerResponse}, url_maps::UrlMapDB
    }, 
    repository::shortener::{
        ShortenerRepository, ShortenerRepositoryTrait
    }
};

pub struct ShortenerUsecase {
    cfg: Arc<ServiceConfig>,
    shortener_repository: Arc<dyn ShortenerRepositoryTrait>,
}

impl ShortenerUsecase  {
    pub fn new(cfg: Arc<ServiceConfig>, repo: Arc<ShortenerRepository>) -> Self {
        Self { 
            shortener_repository: repo,
            cfg: cfg 
        }
    }

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

    pub async fn redirect_url(&self, url_code: String) -> Result<RedirectResponse> {
        let url_map = self.shortener_repository.get_url_mapping(self.cfg.base_url.clone() + url_code.as_str()).await?;
        Ok(RedirectResponse{original_url: url_map.destination_url})
    }
}

