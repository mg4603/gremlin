use std::path::PathBuf;

use http::StatusCode;
use regex::Regex;
use thiserror::Error;
use url::Url;

#[derive(Debug)]
pub struct ScanConfig {
    pub url: Url,
    pub wordlist: PathBuf,
    pub concurrency: usize,

    pub match_status: Option<StatusCode>,
    pub match_regex: Option<Regex>,

    pub filter_size_min: Option<usize>,
    pub filter_size_max: Option<usize>,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),

    #[error("wordlist file not found: {0}")]
    WordlistNotFound(String),

    #[error("concurrency must be greater than zero")]
    InvalidConcurrency,

    #[error("invalid http status code: {0}")]
    InvalidStatusCode(u16),

    #[error("size range invalid: {min}-{max}")]
    InvalidSizeRange { min: usize, max: usize },

    #[error("invalid regex pattern: {0}")]
    InvalidRegexPattern(String),
}

impl ScanConfig {
    pub fn new(
        url: String,
        wordlist: PathBuf,
        concurrency: usize,
        match_status: Option<u16>,
        match_regex: Option<String>,
        filter_size_min: Option<usize>,
        filter_size_max: Option<usize>,
    ) -> Result<Self, ConfigError> {
        let parsed_url = Url::parse(&url).map_err(|_| ConfigError::InvalidUrl(url))?;

        if !wordlist.exists() {
            return Err(ConfigError::WordlistNotFound(
                wordlist.display().to_string(),
            ));
        }

        if concurrency == 0 {
            return Err(ConfigError::InvalidConcurrency);
        }

        let match_status = match match_status {
            Some(code) => {
                Some(StatusCode::from_u16(code).map_err(|_| ConfigError::InvalidStatusCode(code))?)
            }
            None => None,
        };

        if let (Some(min), Some(max)) = (filter_size_min, filter_size_max)
            && min > max
        {
            return Err(ConfigError::InvalidSizeRange { min, max });
        }

        let match_regex = match match_regex {
            Some(pattern) => Some(
                Regex::new(&pattern)
                    .map_err(|e| ConfigError::InvalidRegexPattern(e.to_string()))?,
            ),
            None => None,
        };

        Ok(Self {
            url: parsed_url,
            wordlist,
            concurrency,
            match_status,
            match_regex,
            filter_size_min,
            filter_size_max,
        })
    }
}
