use std::time::Duration;

use bytes::Bytes;
use http::{HeaderMap, StatusCode};

use crate::response::ScanResponse;
use crate::types::Timing;

pub fn response_with_status(status: Option<StatusCode>) -> ScanResponse {
    ScanResponse {
        request_id: 1,
        status,
        headers: HeaderMap::new(),
        body: Some(Bytes::new()),
        timing: Some(Timing {
            total: Duration::from_millis(1),
        }),
        error: None,
    }
}

pub fn response_with_body(body: Option<&str>) -> ScanResponse {
    ScanResponse {
        request_id: 1,
        status: Some(StatusCode::OK),
        headers: HeaderMap::new(),
        body: body.map(|b| Bytes::from(b.to_owned())),
        timing: Some(Timing {
            total: Duration::from_millis(1),
        }),
        error: None,
    }
}

pub fn response_with_size(size: usize) -> ScanResponse {
    ScanResponse {
        request_id: 1,
        status: Some(StatusCode::OK),
        headers: HeaderMap::new(),
        body: Some(Bytes::from(vec![0u8; size])),
        timing: Some(Timing {
            total: Duration::from_millis(1),
        }),
        error: None,
    }
}
