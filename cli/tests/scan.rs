mod common;

use common::assertions::{assert_failure, assert_success};
use common::fixtures::wordlist;
use common::process::run_cli;

#[test]
fn scan_runs_successfully() {
    let output = run_cli(&[
        "scan",
        "--url",
        "https://example.com/FUZZ",
        "--wordlist",
        wordlist().to_str().unwrap(),
    ]);

    assert_success(&output)
}

#[test]
fn scan_fails_with_invalid_wordlist() {
    let output = run_cli(&[
        "scan",
        "--url",
        "https://example.com/FUZZ",
        "--wordlist",
        "wordlist.txt",
    ]);

    assert_failure(&output)
}
