pub mod executor;

use crate::response::ScanResponse;
use crate::result::ScanResult;

pub trait Matcher: Send + Sync {
    fn matches(&self, response: &ScanResponse) -> bool;
}

pub trait Filter: Send + Sync {
    fn allow(&self, result: &ScanResult) -> bool;
}
