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
