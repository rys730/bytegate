use std::sync::Arc;

use async_trait::async_trait;

use crate::app::{
    common::errors::{
        self, Result, ServiceError
    }, 
    infrastructure::postgres::postgres::DB, 
    model::dto::url_maps::UrlMapDB
};

pub struct ShortenerRepository{
    db: Arc<DB>
}

#[async_trait]
pub trait ShortenerRepositoryTrait: Send + Sync {
    async fn create_url_mapping(&self, url_map_dto: UrlMapDB) -> Result<UrlMapDB>;
    async fn get_url_mapping(&self, short_url: String) -> Result<UrlMapDB>;
}

impl ShortenerRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self {db: db}
    }
}

#[async_trait]
impl ShortenerRepositoryTrait for ShortenerRepository {
    async fn create_url_mapping(&self, url_map_dto: UrlMapDB) -> Result<UrlMapDB> {
        let res = sqlx::query_as!(
            UrlMapDB,
            "INSERT INTO url_maps (destination_url, short_url) VALUES ($1, $2) RETURNING *",
            url_map_dto.destination_url,
            url_map_dto.short_url
        )
        .fetch_one(&self.db.pool)
        .await?;
        
        Ok(res)
    }

    async fn get_url_mapping(&self, short_url: String) -> Result<UrlMapDB>{
        let res = sqlx::query_as!(
            UrlMapDB,
            "SELECT * FROM url_maps WHERE short_url = $1",
            short_url
        )
        .fetch_one(&self.db.pool)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ServiceError::NotFoundError(short_url),
            _ => ServiceError::DatabaseError(err)
        })?;
        
        Ok(res)
    }
    
}




