use anyhow::Result;
use dotenv::dotenv;
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Http error")]
    Http(#[from] reqwest::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv()?;

    Ok(())
}
