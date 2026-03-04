use crate::pipeline::Filter;
use crate::result::ScanResult;

pub struct SizeFilter {
    min: usize,
    max: usize,
}

impl SizeFilter {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

impl Filter for SizeFilter {
    fn allow(&self, result: &ScanResult) -> bool {
        result.size() >= self.min && result.size() <= self.max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    use bytes::Bytes;
    use http::{HeaderMap, StatusCode};

    use crate::response::ScanResponse;
    use crate::types::Timing;

    fn build_result(size: usize) -> ScanResult {
        ScanResult {
            request_id: 1,
            response: ScanResponse {
                request_id: 1,
                status: Some(StatusCode::OK),
                headers: HeaderMap::new(),
                body: Some(Bytes::from(vec![0u8; size])),
                timing: Some(Timing {
                    total: Duration::from_millis(5),
                }),
                error: None,
            },
            matched: false,
            notes: Vec::new(),
        }
    }

    #[test]
    fn allows_size_in_range() {
        let filter = SizeFilter::new(100, 200);
        let result = build_result(150);

        assert!(filter.allow(&result));
    }

    #[test]
    fn rejects_size_below_range() {
        let filter = SizeFilter::new(100, 200);
        let result = build_result(50);

        assert!(!filter.allow(&result));
    }

    #[test]
    fn reject_size_above_range() {
        let filter = SizeFilter::new(100, 200);
        let result = build_result(250);

        assert!(!filter.allow(&result));
    }
}
