use wiremock::MockServer;

#[allow(dead_code)]
pub async fn spawn_mock_server() -> MockServer {
    MockServer::start().await
}
