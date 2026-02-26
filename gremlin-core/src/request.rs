use bytes::Bytes;
use http::{HeaderMap, Method};
use url::Url;

use crate::types::RequestId;

#[derive(Debug)]
pub struct ScanRequest {
    pub id: RequestId,
    pub url: Url,
    pub method: Method,
    pub headers: HeaderMap,
    pub body: Option<Bytes>,
}
