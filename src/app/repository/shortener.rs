use crate::app::{
    common::errors::{
        Result, ServiceError
    }, 
    infrastructure::postgres::postgres::DB, 
    model::dto::url_maps::UrlMapDB
};

pub struct ShortenerRepository{
    db: DB
}

pub trait ShortenerRepositoryTrait: Send + Sync {
    async fn create_url_mapping(&self, url_map_dto: UrlMapDB) -> Result<UrlMapDB>;
}

impl ShortenerRepository {
    pub fn new(db: DB) -> Self {
        Self {db: db}
    }
}

impl ShortenerRepositoryTrait for ShortenerRepository {
    async fn create_url_mapping(&self, url_map_dto: UrlMapDB) -> Result<UrlMapDB> {
        let res = sqlx::query_as!(
            UrlMapDB,
            "INSERT INTO url_maps (destination_url, short_url) VALUES ($1, $2) RETURNING *",
            url_map_dto.destination_url,
            url_map_dto.short_url
        )
        .fetch_one(&self.db.pool)
        .await
        .map_err(ServiceError::DatabaseError)?;
        
        Ok(res)
    }
}




