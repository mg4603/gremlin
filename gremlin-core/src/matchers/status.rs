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

    use http::StatusCode;

    use crate::test_helpers::response_with_status;

    #[test]
    fn matches_expected_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = response_with_status(Some(StatusCode::OK));

        assert!(matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_differnt_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = response_with_status(Some(StatusCode::NOT_FOUND));

        assert!(!matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_missing_status() {
        let matcher = StatusMatcher::new(StatusCode::OK);
        let resp = response_with_status(None);

        assert!(!matcher.matches(&resp));
    }
}
