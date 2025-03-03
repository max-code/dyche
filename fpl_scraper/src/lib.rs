mod error;
mod scraper;
mod scrapers;

pub use error::*;
pub use scraper::*;
pub use scrapers::*;

use std::{future::Future, time::Duration};

const DEFAULT_MAX_RETRIES: usize = 5;

async fn with_retry<F, Fut, T, E>(f: F, max_retries: usize) -> Result<T, E>
where
    F: Fn() -> Fut + Clone,
    Fut: Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let mut tries = 0;
    loop {
        match f().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if tries > max_retries {
                    return Err(e);
                }

                if e.to_string().contains("429") {
                    let delay = Duration::from_millis(2u64.pow(tries as u32) * 500);
                    tracing::warn!(
                        "Rate limited (429). Retrying after {:?} (retry {})",
                        delay,
                        tries + 1
                    );
                    tokio::time::sleep(delay).await;
                    tries += 1;
                    continue;
                }

                return Err(e);
            }
        }
    }
}
