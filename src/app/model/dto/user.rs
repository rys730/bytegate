use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Default)]
pub struct UserDB {
    pub id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub username: String,
    pub password: String,
    pub session: String
}

#[derive(Deserialize)]
pub struct RegisterUserRequest {
    pub username: String,
    pub password: String
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub session: String,
}