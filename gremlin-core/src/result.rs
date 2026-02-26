use crate::response::ScanResponse;
use crate::types::RequestId;

#[derive(Debug)]
pub struct ScanResult {
    pub request_id: RequestId,
    pub response: ScanResponse,
    pub matched: bool,
    pub notes: Vec<String>,
}
