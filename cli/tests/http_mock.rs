use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

mod common;

use common::assertions::assert_success;
use common::fixtures::wordlist;
use common::mock::spawn_mock_server;
use common::process::run_cli;

#[tokio::test]
async fn test_basic_http_request() {
    let server = spawn_mock_server().await;

    Mock::given(method("GET"))
        .and(path("/test"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let url = format!("{}/test", server.uri());

    let output = run_cli(&[
        "--quiet",
        "scan",
        "--url",
        &url,
        "--wordlist",
        wordlist().to_str().unwrap(),
        "--concurrency",
        "20",
    ]);
    assert_success(&output)
}
