use std::sync::atomic::{AtomicU64, Ordering};

use http::{HeaderMap, Method};
use thiserror::Error;
use url::Url;

use crate::config::ScanConfig;
use crate::request::ScanRequest;
use crate::types::RequestId;
use crate::wordlist::WordlistReader;

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("wordlist io error")]
    Io(#[from] std::io::Error),

    #[error("generated url is invalid: {0}")]
    InvalidGeneratedUrl(String),
}

pub struct JobGenerator {
    config: ScanConfig,
    reader: WordlistReader,
    counter: AtomicU64,
}

impl JobGenerator {
    pub async fn new(config: ScanConfig) -> Result<Self, GeneratorError> {
        let reader = WordlistReader::open(&config.wordlist).await?;

        Ok(Self {
            config,
            reader,
            counter: AtomicU64::new(1),
        })
    }

    pub async fn next(&mut self) -> Result<Option<ScanRequest>, GeneratorError> {
        if let Some(entry) = self.reader.next().await? {
            let id: RequestId = self.counter.fetch_add(1, Ordering::Relaxed);

            let fuzzed_url = self.config.url.as_str().replace("FUZZ", &entry);

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
