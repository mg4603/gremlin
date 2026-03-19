use thiserror::Error;

#[derive(Debug, Error, Clone)]
pub enum ResponseError {
    #[error("request timed out")]
    Timeout,

    #[error("dns resolution failed")]
    DnsFailure,

    #[error("connection failed")]
    ConnectionFailure,

    #[error("tls handshake failed")]
    TlsFailure,

    #[error("invalid http response")]
    InvalidResponse,

    #[error("unexpected transport error: {0}")]
    Other(String),
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),

    #[error("wordlist file not found: {0}")]
    WordlistNotFound(String),

    #[error("invalid concurrency: {0} (must be > 0)")]
    InvalidConcurrency(usize),

    #[error("invalid number of requests: {0} (must be > 0)")]
    InvalidNumberOfRequests(usize),

    #[error("invalid http status code: {0}")]
    InvalidStatusCode(u16),

    #[error("invalid size range: min={min}, max={max} (min mut be <= max)")]
    InvalidSizeRange { min: usize, max: usize },

    #[error("invalid regex: {0}")]
    InvalidRegex(String),

    #[error("invalid rate limit: {0} (must be > 0)")]
    InvalidRateLimit(u64),
}

#[derive(Debug, Error)]
pub enum GeneratorError {
    #[error("wordlist io error")]
    Io(#[from] std::io::Error),

    #[error("invalid generated url from base={base} input={input}: {source}")]
    InvalidUrl {
        base: String,
        input: String,
        #[source]
        source: url::ParseError,
    },
}
