use std::path::{Path, PathBuf};

use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct ScanConfig {
    pub url: Url,
    pub wordlist: PathBuf,
    pub concurrency: usize,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),

    #[error("wordlist file not found: {0}")]
    WordlistNotFound(String),

    #[error("concurrency must be greater than zero")]
    InvalidConcurrency,
}

impl ScanConfig {
    pub fn new(url: &str, wordlist: &Path, concurrency: usize) -> Result<Self, ConfigError> {
        let parsed_url = Url::parse(url).map_err(|_| ConfigError::InvalidUrl(url.to_string()))?;

        if !wordlist.exists() {
            return Err(ConfigError::WordlistNotFound(
                wordlist.display().to_string(),
            ));
        }

        if concurrency == 0 {
            return Err(ConfigError::InvalidConcurrency);
        }

        Ok(Self {
            url: parsed_url,
            wordlist: wordlist.to_path_buf(),
            concurrency,
        })
    }
}
