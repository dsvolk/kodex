//! A denied credential refresh ends 401 recovery without changing stored credentials.

use std::sync::Arc;

use anyhow::Result;
use core_test_support::responses::start_mock_server;
use core_test_support::test_kodex::test_kodex;
use core_test_support::wait_for_event;
use kodex_core::TurnInputRequest;
use kodex_http_client::HttpClientFactory;
use kodex_http_client::NetworkPolicyController;
use kodex_http_client::NetworkPolicyDenied;
use kodex_http_client::OutboundProxyPolicy;
use kodex_login::AuthCredentialsStoreMode;
use kodex_login::AuthKeyringBackendKind;
use kodex_login::AuthManager;
use kodex_login::AuthRouteConfig;
use kodex_protocol::protocol::EventMsg;
use kodex_protocol::user_input::UserInput;
use pretty_assertions::assert_eq;
use serde_json::json;
use tempfile::TempDir;
use wiremock::Mock;
use wiremock::ResponseTemplate;
use wiremock::matchers::header;
use wiremock::matchers::method;
use wiremock::matchers::path;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn denied_oauth_refresh_terminates_401_recovery_and_preserves_auth() -> Result<()> {
    let home = Arc::new(TempDir::new()?);
    let auth_json = serde_json::to_vec(&json!({
        "auth_mode": "chatgpt",
        "tokens": {
            "id_token": "e30.e30.signature",
            "access_token": "current-access-token",
            "refresh_token": "current-refresh-token",
            "account_id": "workspace-one"
        },
        "last_refresh": chrono::Utc::now()
    }))?;
    std::fs::write(home.path().join("auth.json"), &auth_json)?;
    let auth = AuthManager::shared(
        home.path().to_path_buf(),
        /*enable_kodex_api_key_env*/ false,
        AuthCredentialsStoreMode::File,
        /*forced_chatgpt_workspace_id*/ None,
        /*chatgpt_base_url*/ None,
        AuthKeyringBackendKind::default(),
        AuthRouteConfig::from_http_client_factory(
            HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault)
                .with_network_policy(NetworkPolicyController::default().policy()),
        ),
    )
    .await;
    let original = auth.auth_cached().expect("stored ChatGPT auth");
    let server = start_mock_server().await;
    Mock::given(method("POST"))
        .and(path("/v1/responses"))
        .and(header("authorization", "Bearer current-access-token"))
        .and(header("chatgpt-account-id", "workspace-one"))
        .respond_with(ResponseTemplate::new(401))
        .expect(/*r*/ 2) // Initial request and the existing disk-reload recovery step.
        .mount(&server)
        .await;
    let mut builder = test_kodex().with_home(home).with_auth_manager(auth.clone());
    let test = builder.build_with_auto_env(&server).await?;
    test.kodex
        .start_or_steer_turn(TurnInputRequest::user_input(vec![UserInput::Text {
            text: "hello".into(),
            text_elements: Vec::new(),
        }]))
        .await?;
    let mut policy_error = None;
    wait_for_event(&test.kodex, |event| {
        if let EventMsg::Error(error) = event {
            policy_error.get_or_insert_with(|| error.message.clone());
        }
        matches!(event, EventMsg::TurnComplete(_))
    })
    .await;
    assert_eq!(
        policy_error,
        Some(format!("Fatal error: {}", NetworkPolicyDenied::Unavailable))
    );
    assert_eq!(auth.auth().await, Some(original));
    assert_eq!(
        std::fs::read(test.home.path().join("auth.json"))?,
        auth_json
    );
    server.verify().await;
    Ok(())
}
