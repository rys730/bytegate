use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct UrlMap {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expired_at: Option<DateTime<Utc>>,
    pub destination_url: String,
    pub short_url: String,
}