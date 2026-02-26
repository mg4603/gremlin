use crate::{request::ScanRequest, response::ScanResponse};

#[derive(Debug, Clone)]
pub struct ScanResult {
    pub request: ScanRequest,
    pub response: ScanResponse,
    pub matched: bool,
    pub notes: Vec<String>,
}
