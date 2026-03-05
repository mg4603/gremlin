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

    use crate::test_helpers::response_with_body;

    #[test]
    fn matches_when_regex_found() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = response_with_body(Some("admin panel"));

        assert!(matcher.matches(&resp));
    }

    #[test]
    fn does_not_match_when_regex_missing() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = response_with_body(Some("not a match"));

        assert!(!matcher.matches(&resp));
    }

    #[test]
    fn returns_false_when_body_missing() {
        let matcher = RegexMatcher::new("admin").unwrap();
        let resp = response_with_body(None);

        assert!(!matcher.matches(&resp));
    }
}
