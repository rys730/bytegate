use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct ShortenerRequest {
    pub url: String,
}

#[derive(Debug, Serialize)]
pub struct ShortenerResponse {
    pub short_url: String,
}

pub struct RedirectResponse {
    pub original_url: String
}