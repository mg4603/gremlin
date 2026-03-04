use http::StatusCode;

use crate::pipeline::Matcher;
use crate::response::ScanResponse;

pub struct StatusMatcher {
    status: StatusCode,
}

impl StatusMatcher {
    pub fn new(status: StatusCode) -> Self {
        Self { status }
    }
}

impl Matcher for StatusMatcher {
    fn matches(&self, response: &ScanResponse) -> bool {
        match response.status {
            Some(status) => status == self.status,
            None => false,
        }
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

    fn build_response(status: Option<StatusCode>) -> ScanResponse {
        ScanResponse {
            request_id: 1,
            status,
            headers: HeaderMap::new(),
            body: Some(Bytes::new()),
            timing: Some(Timing {
                total: Duration::from_millis(10),
            }),
            error: None,
        }
    }

    #[test]
    fn matches_expected_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = build_response(Some(StatusCode::OK));

        assert!(matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_differnt_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = build_response(Some(StatusCode::NOT_FOUND));

        assert!(!matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_missing_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = build_response(None);

        assert!(!matcher.matches(&resp));
    }
}
