use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;
use http::{HeaderMap, Method};
use url::Url;

use crate::config::{BenchmarkConfig, ScanConfig};
use crate::error::GeneratorError;
use crate::request::ScanRequest;
use crate::types::RequestId;
use crate::wordlist::WordlistReader;

#[async_trait]
pub trait JobGenerator {
    async fn next(&mut self) -> Result<Option<ScanRequest>, GeneratorError>;
}

pub struct ScanJobGenerator {
    url: Url,
    reader: WordlistReader,
    counter: AtomicU64,
}

impl ScanJobGenerator {
    pub async fn new(config: ScanConfig) -> Result<Self, GeneratorError> {
        let reader = WordlistReader::open(&config.wordlist).await?;

        Ok(Self {
            url: config.url,
            reader,
            counter: AtomicU64::new(1),
        })
    }
}

#[async_trait]
impl JobGenerator for ScanJobGenerator {
    async fn next(&mut self) -> Result<Option<ScanRequest>, GeneratorError> {
        if let Some(entry) = self.reader.next().await? {
            let id: RequestId = self.counter.fetch_add(1, Ordering::Relaxed);

            let fuzzed_url = self.url.as_str().replace("FUZZ", &entry);

            let parsed_url = Url::parse(&fuzzed_url)
                .map_err(|_| GeneratorError::InvalidGeneratedUrl(fuzzed_url.clone()))?;
            Ok(Some(ScanRequest {
                id,
                url: parsed_url,
                method: Method::GET,
                headers: HeaderMap::new(),
                body: None,
            }))
        } else {
            Ok(None)
        }
    }
}

pub struct BenchmarkJobGenerator {
    url: Url,
    requests: usize,
    counter: AtomicU64,
}

impl BenchmarkJobGenerator {
    pub fn new(config: BenchmarkConfig) -> Result<Self, GeneratorError> {
        Ok(Self {
            url: config.url,
            requests: config.requests,
            counter: AtomicU64::new(0),
        })
    }
}

#[async_trait]
impl JobGenerator for BenchmarkJobGenerator {
    async fn next(&mut self) -> Result<Option<ScanRequest>, GeneratorError> {
        let count = self.counter.load(Ordering::Relaxed) as usize;
        if count < self.requests {
            let url_str = format!("{}/{}", self.url.as_str().trim_end_matches('/'), count);
            let parsed_url = Url::parse(&url_str)
                .map_err(|_| GeneratorError::InvalidGeneratedUrl(url_str.clone()))?;
            self.counter.fetch_add(1, Ordering::Relaxed);

            Ok(Some(ScanRequest {
                id: count as RequestId,
                url: parsed_url,
                method: Method::GET,
                headers: HeaderMap::new(),
                body: None,
            }))
        } else {
            Ok(None)
        }
    }
}
