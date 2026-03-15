use std::path::PathBuf;

use http::StatusCode;
use regex::Regex;
use thiserror::Error;
use url::Url;

use crate::filters::size::SizeFilter;
use crate::matchers::{regex::RegexMatcher, status::StatusMatcher};
use crate::pipeline::{Filter, Matcher};

#[derive(Debug)]
pub struct ScanConfig {
    pub url: Url,
    pub wordlist: PathBuf,
    pub concurrency: usize,

    pub match_status: Option<StatusCode>,
    pub match_regex: Option<Regex>,

    pub filter_size_min: Option<usize>,
    pub filter_size_max: Option<usize>,

    pub rate_limit: Option<u64>,
}

#[derive(Debug)]
pub struct BenchmarkConfig {
    pub url: Url,
    pub requests: usize,
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

    #[error("number of requests must be greater than zero")]
    InvalidNumberOfRequests,

    #[error("invalid http status code: {0}")]
    InvalidStatusCode(u16),

    #[error("size range invalid: {min}-{max}")]
    InvalidSizeRange { min: usize, max: usize },

    #[error("invalid regex pattern: {0}")]
    InvalidRegexPattern(String),

    #[error("invalid request rate limit")]
    InvalidRateLimit,
}

impl ScanConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        url: String,
        wordlist: PathBuf,
        concurrency: usize,
        match_status: Option<u16>,
        match_regex: Option<String>,
        filter_size_min: Option<usize>,
        filter_size_max: Option<usize>,
        rate_limit: Option<u64>,
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

        if let Some(rate) = rate_limit
            && rate == 0
        {
            return Err(ConfigError::InvalidRateLimit);
        }

        Ok(Self {
            url: parsed_url,
            wordlist,
            concurrency,
            match_status,
            match_regex,
            filter_size_min,
            filter_size_max,
            rate_limit,
        })
    }

    pub fn build_matchers(&self) -> Vec<Box<dyn Matcher>> {
        let mut matchers: Vec<Box<dyn Matcher>> = Vec::new();

        if let Some(status) = self.match_status {
            matchers.push(Box::new(StatusMatcher::new(status)));
        }

        if let Some(regex) = &self.match_regex {
            matchers.push(Box::new(RegexMatcher::new(regex.clone())));
        }
        matchers
    }

    pub fn build_filters(&self) -> Vec<Box<dyn Filter>> {
        let mut filters: Vec<Box<dyn Filter>> = Vec::new();

        if let (Some(min), Some(max)) = (self.filter_size_min, self.filter_size_max) {
            filters.push(Box::new(SizeFilter::new(min, max)));
        }
        filters
    }
}

impl BenchmarkConfig {
    pub fn new(url: String, requests: usize, concurrency: usize) -> Result<Self, ConfigError> {
        let parsed_url = Url::parse(&url).map_err(|_| ConfigError::InvalidUrl(url))?;

        if concurrency == 0 {
            return Err(ConfigError::InvalidConcurrency);
        }

        if requests == 0 {
            return Err(ConfigError::InvalidNumberOfRequests);
        }

        Ok(Self {
            url: parsed_url,
            requests,
            concurrency,
        })
    }
}
