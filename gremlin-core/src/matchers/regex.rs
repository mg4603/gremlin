use regex::Regex;

use crate::pipeline::Matcher;
use crate::response::ScanResponse;

pub struct RegexMatcher {
    regex: Regex,
}

impl RegexMatcher {
    pub fn new(pattern: &str) -> Result<Self, regex::Error> {
        let regex = Regex::new(pattern)?;

        Ok(Self { regex })
    }
}

impl Matcher for RegexMatcher {
    fn matches(&self, response: &ScanResponse) -> bool {
        match &response.body {
            Some(body) => {
                let text = String::from_utf8_lossy(body);
                self.regex.is_match(&text)
            }
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

    fn build_response(body: Option<&str>) -> ScanResponse {
        ScanResponse {
            request_id: 1,
            status: Some(StatusCode::OK),
            headers: HeaderMap::new(),
            body: body.map(|b| Bytes::from(b.to_owned())),
            timing: Some(Timing {
                total: Duration::from_millis(10),
            }),
            error: None,
        }
    }

    #[test]
    fn matches_when_regex_found() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = build_response(Some("admin panel"));

        assert!(matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_when_regex_missing() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = build_response(Some("not a match"));

        assert!(!matcher.matches(&resp));
    }

    #[test]
    fn returns_false_when_body_missing() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = build_response(None);

        assert!(!matcher.matches(&resp));
    }
}
