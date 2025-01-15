use firestore_db_and_auth::documents::WriteOptions; // Import WriteOptions
use firestore_db_and_auth::{Credentials, ServiceSession};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct URLRecord {
    pub long_url: String,
    pub expiry: i64,
}

pub async fn save_url(
    id: &str,
    long_url: &str,
    expiry: i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let credentials = Credentials::new("path/to/serviceAccount.json").await?;
    let session = ServiceSession::new(credentials).await?;

    let record = URLRecord {
        long_url: long_url.to_string(),
        expiry,
    };

    firestore_db_and_auth::documents::write(
        &session,
        &format!("urls/{}", id),
        None::<&str>,
        &record,
        WriteOptions::default(),
    )
    .await?;

    Ok(())
}

pub async fn get_url(id: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let credentials = Credentials::new("path/to/serviceAccount.json").await?;
    let session = ServiceSession::new(credentials).await?;

    match firestore_db_and_auth::documents::read::<URLRecord>(
        &session, "urls", // Collection path
        id,     // Document ID
    )
    .await
    {
        Ok(doc) => Ok(Some(doc.long_url)),
        Err(e) if e.to_string().contains("not found") => Ok(None), // Handle "not found" errors
        Err(e) => Err(Box::new(e)),
    }
}
