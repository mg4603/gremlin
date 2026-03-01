use std::time::Instant;

use http::HeaderMap;
use reqwest::Client;
use thiserror::Error;

use gremlin_core::error::ResponseError;
use gremlin_core::request::ScanRequest;
use gremlin_core::response::ScanResponse;
use gremlin_core::types::Timing;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("failed to build HTTP client")]
    ClientBuild(#[from] reqwest::Error),
}

pub struct HttpEngine {
    client: Client,
}

impl HttpEngine {
    pub fn new() -> Result<Self, EngineError> {
        let client = Client::builder().build()?;

        Ok(Self { client })
    }

    pub async fn execute(&self, request: ScanRequest) -> ScanResponse {
        let start = Instant::now();

        let result = self
            .client
            .request(request.method.clone(), request.url.clone())
            .headers(request.headers.clone())
            .body(request.body.unwrap_or_default())
            .send()
            .await;

        match result {
            Ok(resp) => {
                let status = Some(resp.status());
                let headers = resp.headers().clone();

                let body = resp.bytes().await.ok();

                ScanResponse {
                    request_id: request.id,
                    status,
                    body,
                    headers,
                    error: None,
                    timing: Some(Timing {
                        total: start.elapsed(),
                    }),
                }
            }
            Err(e) => {
                let mapped = map_reqwest_error(&e);

                ScanResponse {
                    request_id: request.id,
                    status: None,
                    headers: HeaderMap::new(),
                    body: None,
                    error: Some(mapped),
                    timing: Some(Timing {
                        total: start.elapsed(),
                    }),
                }
            }
        }
    }
}

fn map_reqwest_error(err: &reqwest::Error) -> ResponseError {
    if err.is_timeout() {
        ResponseError::Timeout
    } else if err.is_connect() {
        ResponseError::ConnectionFailure
    } else if err.is_request() {
        ResponseError::InvalidResponse
    } else {
        ResponseError::Other(err.to_string())
    }
}
