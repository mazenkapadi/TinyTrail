use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub long_url: String,
    pub validity: u8,
}

// Response payload for shortened URL
#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}
