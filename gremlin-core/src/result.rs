use crate::response::ScanResponse;
use crate::types::RequestId;

#[derive(Debug)]
pub struct ScanResult {
    pub request_id: RequestId,
    pub response: ScanResponse,
    pub matched: bool,
    pub notes: Vec<String>,
}

impl ScanResult {
    pub fn size(&self) -> usize {
        self.response.body.as_ref().map(|b| b.len()).unwrap_or(0)
    }
}
