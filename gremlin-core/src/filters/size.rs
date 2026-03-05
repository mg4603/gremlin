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

    use crate::test_helpers::response_with_size;

    fn build_result(size: usize) -> ScanResult {
        ScanResult {
            request_id: 1,
            response: response_with_size(size),
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
