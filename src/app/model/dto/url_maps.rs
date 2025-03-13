use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, FromRow, Default)]
pub struct UrlMapDB {
    pub id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub expired_at: Option<DateTime<Utc>>,
    pub destination_url: String,
    pub short_url: String,
}