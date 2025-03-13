use std::sync::Arc;

use async_trait::async_trait;

use crate::app::infrastructure::postgres::postgres::DB;
use crate::app::common::errors::Result;

pub struct UserRepository{
    db: Arc<DB>
}

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(&self) -> Result<String>;
}

impl UserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self {db: db}
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository{
    async fn create_user(&self) -> Result<String>{
        Ok(String::new())
    }
}