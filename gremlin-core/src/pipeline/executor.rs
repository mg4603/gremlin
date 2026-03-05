use crate::pipeline::{Filter, Matcher};
use crate::response::ScanResponse;
use crate::result::ScanResult;

pub struct Pipeline {
    matchers: Vec<Box<dyn Matcher>>,
    filters: Vec<Box<dyn Filter>>,
}

impl Pipeline {
    pub fn new(matchers: Vec<Box<dyn Matcher>>, filters: Vec<Box<dyn Filter>>) -> Self {
        Self { matchers, filters }
    }

    pub fn process(&self, response: ScanResponse) -> Option<ScanResult> {
        let matched = self.matchers.iter().any(|m| m.matches(&response));
        if !matched {
            return None;
        }

        let result = ScanResult {
            request_id: response.request_id,
            response,
            matched,
            notes: Vec::new(),
        };

        let allowed = self.filters.iter().all(|f| f.allow(&result));
        if allowed { Some(result) } else { None }
    }
}
