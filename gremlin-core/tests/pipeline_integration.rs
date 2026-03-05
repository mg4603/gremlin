use http::StatusCode;

use gremlin_core::test_helpers::response_with_status;

use gremlin_core::matchers::status::StatusMatcher;
use gremlin_core::pipeline::executor::Pipeline;

#[test]
fn pipeline_emits_results_when_match_passes() {
    let matcher = StatusMatcher::new(StatusCode::OK);

    let pipeline = Pipeline::new(vec![Box::new(matcher)], vec![]);

    let response = response_with_status(Some(StatusCode::OK));

    let result = pipeline.process(response);

    assert!(result.is_some());
}

#[test]
fn pipeline_suppresses_when_match_fails() {
    let matcher = StatusMatcher::new(StatusCode::NOT_FOUND);

    let pipeline = Pipeline::new(vec![Box::new(matcher)], vec![]);

    let response = response_with_status(Some(StatusCode::OK));

    let result = pipeline.process(response);

    assert!(result.is_none());
}
