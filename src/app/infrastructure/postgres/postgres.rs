use sqlx::{Pool, Postgres};

use crate::app::common::config::DBConfig;

pub struct DB {
    pool: Pool<Postgres>
}

impl DB {
    pub async fn new(cfg: DBConfig) -> Result<Self, sqlx::Error>{
        let database_url = format!(
            "postgres://{}:{}@{}/{}", 
            cfg.db_user, 
            cfg.db_pass, 
            cfg.db_host, 
            cfg.db_name);
        let pool = Pool::<Postgres>::connect(&database_url).await?;
        Ok(Self{pool: pool})
    }
}