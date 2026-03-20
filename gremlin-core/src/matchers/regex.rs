use regex::bytes::Regex;

use crate::pipeline::Matcher;
use crate::response::ScanResponse;

pub struct RegexMatcher {
    regex: Regex,
}

impl RegexMatcher {
    pub fn new(regex: Regex) -> Self {
        Self { regex }
    }
}

impl Matcher for RegexMatcher {
    fn matches(&self, response: &ScanResponse) -> bool {
        match &response.body {
            Some(body) => self.regex.is_match(body),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::test_helpers::response_with_body;

    #[test]
    fn matches_when_regex_found() {
        let regex = Regex::new("admin").unwrap();
        let matcher = RegexMatcher::new(regex);
        let resp = response_with_body(Some("admin panel"));

        assert!(matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_when_regex_missing() {
        let regex = Regex::new("admin").unwrap();
        let matcher = RegexMatcher::new(regex);
        let resp = response_with_body(Some("not a match"));

        assert!(!matcher.matches(&resp));
    }

    #[test]
    fn returns_false_when_body_missing() {
        let regex = Regex::new("admin").unwrap();
        let matcher = RegexMatcher::new(regex);
        let resp = response_with_body(None);

        assert!(!matcher.matches(&resp));
    }
}
