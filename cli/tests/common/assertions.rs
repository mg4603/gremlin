use std::process::Output;

#[allow(dead_code)]
pub fn assert_success(output: &Output) {
    assert!(output.status.success());
}

#[allow(dead_code)]
pub fn assert_failure(output: &Output) {
    assert!(!output.status.success());
}
