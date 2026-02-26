use bytes::Bytes;
use http::{HeaderMap, StatusCode};

use crate::error::ResponseError;
use crate::types::{RequestId, Timing};

#[derive(Debug)]
pub struct ScanResponse {
    pub request_id: RequestId,
    pub status: Option<StatusCode>,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
    pub timing: Option<Timing>,
    pub error: Option<ResponseError>,
}
