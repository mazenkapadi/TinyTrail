use chrono::{Duration, Utc};
use uuid::Uuid;

pub fn generate_short_id() -> String {
    Uuid::new_v4().to_string()[..8].to_string()
}

pub fn calculate_expiry(validity: u8) -> i64 {
    match validity {
        1 => (Utc::now() + Duration::hours(24)).timestamp(),
        2 => (Utc::now() + Duration::days(7)).timestamp(),
        _ => i64::MAX,
    }
}
