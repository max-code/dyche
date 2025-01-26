use thiserror::Error;

#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("FPL API error: {0}")]
    FplApiError(String),
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
