use waka::{WakaTimeClient, WakaTimeClientBuilder};
use wiremock::MockServer;

/// Builds a [`WakaTimeClient`] that talks to the given mock server.
pub fn client_for(server: &MockServer) -> WakaTimeClient {
    WakaTimeClientBuilder::new_with_api_key("test-api-key")
        .with_base_url(server.uri())
        .build()
        .expect("failed to build client")
}
