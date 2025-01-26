use fpl_api::FplClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("FPL API error: {0}")]
    FplApiError(#[from] FplClientError),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
