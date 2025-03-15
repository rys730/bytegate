use std::sync::Arc;

use async_trait::async_trait;

use crate::app::{common::errors::Result, infrastructure::postgres::postgres::DB, model::dto::user::UserDB};


pub struct UserRepository {
    db: Arc<DB>
}

#[async_trait]
pub trait UserRepositoryTrait: Send + Sync {
    async fn create_user(&self, user: UserDB) -> Result<UserDB>;
}


impl UserRepository {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db: db }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository{
    async fn create_user(&self, user: UserDB) -> Result<UserDB>{
        let res = sqlx::query_as!(
            UserDB,
            "INSERT INTO users (username, password, session) VALUES ($1, $2, $3) RETURNING *",
            user.username,
            user.password,
            user.session
        )
        .fetch_one(&self.db.pool)
        .await?;

        Ok(res)
    }
}