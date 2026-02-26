use bytes::Bytes;
use http::HeaderMap;

use crate::types::{RequestId, Timing};

#[derive(Debug, Clone)]
pub struct ScanResponse {
    pub request_id: RequestId,
    pub status: Option<u16>,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
    pub timing: Option<Timing>,
    pub error: Option<String>,
}
