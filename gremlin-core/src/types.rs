use std::time::Duration;

pub type RequestId = u64;

#[derive(Debug, Clone)]
pub struct Timing {
    pub duration: Duration,
}
