use std::sync::Arc;

use crate::app::repository::shortener::{ShortenerRepository, ShortenerRepositoryTrait};

pub trait ShortenerUsecaseTrait: Send + Sync {
    fn create_short_url(&self, url: String) -> String;
}

pub struct ShortenerUsecase {
    shortener_repository: Arc<ShortenerRepository>,
}

impl ShortenerUsecase {
    pub fn new(repo: Arc<ShortenerRepository>) -> Self {
        Self { shortener_repository: repo }
    }
}

impl ShortenerUsecaseTrait for ShortenerUsecase {
    fn create_short_url(&self, url: String) -> String {
        let short_url = self.shortener_repository.create_url_mapping(url);
        short_url
    }
}

