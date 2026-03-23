use std::process::Output;

pub fn assert_success(output: &Output) {
    assert!(output.status.success());
}

pub fn assert_failure(output: &Output) {
    assert!(!output.status.success());
}
