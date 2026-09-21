//! Test-only helpers exposed for cross-crate integration tests.
//!
//! Production code should not depend on this module.
//! We prefer this to using a crate feature to avoid building multiple
//! permutations of the crate.

use std::path::PathBuf;
use std::sync::Arc;

use kodex_exec_server::EnvironmentManager;
use kodex_extension_api::LoadInstructionsFuture;
use kodex_extension_api::LoadedUserInstructions;
use kodex_extension_api::UserInstructionsProvider;
use kodex_http_client::HttpClientFactory;
use kodex_http_client::OutboundProxyPolicy;
use kodex_login::AuthManager;
use kodex_login::KodexAuth;
use kodex_model_provider::create_model_provider;
use kodex_model_provider_info::ModelProviderInfo;
use kodex_models_manager::bundled_models_response;
use kodex_models_manager::collaboration_mode_presets;
use kodex_models_manager::manager::SharedModelsManager;
use kodex_models_manager::test_support::construct_model_info_offline_for_tests;
use kodex_models_manager::test_support::get_model_offline_for_tests;
use kodex_protocol::ThreadId;
use kodex_protocol::config_types::CollaborationModeMask;
use kodex_protocol::mcp::ClientMcpExtensions;
use kodex_protocol::mcp::OPENAI_FORM_EXTENSION_ID;
use kodex_protocol::openai_models::ModelInfo;
use kodex_protocol::openai_models::ModelPreset;
use kodex_protocol::protocol::SessionSource;
use once_cell::sync::Lazy;

use crate::ThreadManager;
use crate::config::Config;
use crate::responses_metadata::KodexResponsesMetadata;
use crate::responses_metadata::KodexResponsesRequestKind;
use crate::responses_metadata::subagent_header_value;
use crate::responses_metadata::subagent_metadata_kind;
use crate::thread_manager;
use crate::unified_exec;

static TEST_MODEL_PRESETS: Lazy<Vec<ModelPreset>> = Lazy::new(|| {
    let mut response = bundled_models_response()
        .unwrap_or_else(|err| panic!("bundled models.json should parse: {err}"));
    response.models.sort_by_key(|model| model.priority);
    let mut presets: Vec<ModelPreset> = response.models.into_iter().map(Into::into).collect();
    ModelPreset::mark_default_by_picker_visibility(&mut presets);
    presets
});

/// Reattaches request-only observations to a completed turn's history for capture assertions.
/// Tests inspect this separately from the destination-filtered HTTP/WS request.
pub async fn history_with_tool_call_metadata(
    thread: &crate::KodexThread,
) -> Vec<kodex_protocol::models::ResponseItem> {
    let history = thread.conversation_history_snapshot().await;
    let mut items = history.items().cloned().collect::<Vec<_>>();
    thread
        .session
        .services
        .executed_tool_calls
        .attach_to_prompt(&mut items, &mut Default::default());
    items
}

/// Test-only provider that supplies no user instructions.
#[derive(Debug, Default)]
pub struct EmptyUserInstructionsProvider;

impl UserInstructionsProvider for EmptyUserInstructionsProvider {
    fn load_user_instructions(&self) -> LoadInstructionsFuture<'_> {
        Box::pin(async { LoadedUserInstructions::default() })
    }
}

pub fn set_thread_manager_test_mode(enabled: bool) {
    thread_manager::set_thread_manager_test_mode_for_tests(enabled);
}

pub fn set_deterministic_process_ids(enabled: bool) {
    unified_exec::set_deterministic_process_ids_for_tests(enabled);
}

pub fn auth_manager_from_auth(auth: KodexAuth) -> Arc<AuthManager> {
    AuthManager::from_auth_for_testing(auth)
}

pub fn auth_manager_from_auth_with_home(auth: KodexAuth, kodex_home: PathBuf) -> Arc<AuthManager> {
    AuthManager::from_auth_for_testing_with_home(auth, kodex_home)
}

pub fn with_code_mode_host_program(
    thread_manager: ThreadManager,
    host_program: PathBuf,
    config: &crate::config::Config,
) -> ThreadManager {
    thread_manager.with_code_mode_host_program_for_tests(host_program, config)
}

pub fn thread_manager_with_models_provider(
    auth: KodexAuth,
    provider: ModelProviderInfo,
) -> ThreadManager {
    ThreadManager::with_models_provider_for_tests(auth, provider)
}

pub fn thread_manager_with_models_provider_and_home(
    auth: KodexAuth,
    provider: ModelProviderInfo,
    kodex_home: PathBuf,
    environment_manager: Arc<EnvironmentManager>,
) -> ThreadManager {
    ThreadManager::with_models_provider_and_home_for_tests(
        auth,
        provider,
        kodex_home,
        environment_manager,
    )
}

pub async fn start_thread_with_user_shell_override(
    thread_manager: &ThreadManager,
    config: Config,
    user_shell_override: crate::shell::Shell,
    supports_openai_form_elicitation: bool,
) -> kodex_protocol::error::Result<crate::NewThread> {
    thread_manager
        .start_thread_with_user_shell_override_for_tests(
            config,
            user_shell_override,
            ClientMcpExtensions::new(
                supports_openai_form_elicitation
                    .then(|| (OPENAI_FORM_EXTENSION_ID.to_string(), serde_json::json!({}))),
            ),
        )
        .await
}

pub async fn resume_thread_from_rollout_with_user_shell_override(
    thread_manager: &ThreadManager,
    config: Config,
    rollout_path: PathBuf,
    auth_manager: Arc<AuthManager>,
    user_shell_override: crate::shell::Shell,
    supports_openai_form_elicitation: bool,
) -> kodex_protocol::error::Result<crate::NewThread> {
    thread_manager
        .resume_thread_from_rollout_with_user_shell_override_for_tests(
            config,
            rollout_path,
            auth_manager,
            user_shell_override,
            ClientMcpExtensions::new(
                supports_openai_form_elicitation
                    .then(|| (OPENAI_FORM_EXTENSION_ID.to_string(), serde_json::json!({}))),
            ),
        )
        .await
}

pub fn models_manager_with_provider(
    kodex_home: PathBuf,
    auth_manager: Arc<AuthManager>,
    provider: ModelProviderInfo,
) -> SharedModelsManager {
    let provider = create_model_provider(provider, Some(auth_manager));
    provider.models_manager(kodex_home, /*config_model_catalog*/ None)
}

pub fn default_http_client_factory() -> HttpClientFactory {
    HttpClientFactory::new(OutboundProxyPolicy::ReqwestDefault)
}

pub fn get_model_offline(model: Option<&str>) -> String {
    get_model_offline_for_tests(model)
}

pub fn construct_model_info_offline(model: &str, config: &Config) -> ModelInfo {
    construct_model_info_offline_for_tests(model, &config.to_models_manager_config())
}

#[derive(Clone, Copy)]
pub enum TestKodexResponsesRequestKind {
    Turn,
    Prewarm,
    WebsocketConnection,
}

#[allow(clippy::too_many_arguments)]
pub fn responses_metadata(
    installation_id: &str,
    session_id: &str,
    thread_id: &str,
    turn_id: Option<&str>,
    window_id: String,
    session_source: &SessionSource,
    parent_thread_id: Option<ThreadId>,
    request_kind: TestKodexResponsesRequestKind,
) -> KodexResponsesMetadata {
    let request_kind = match request_kind {
        TestKodexResponsesRequestKind::Turn => Some(KodexResponsesRequestKind::Turn),
        TestKodexResponsesRequestKind::Prewarm => Some(KodexResponsesRequestKind::Prewarm),
        TestKodexResponsesRequestKind::WebsocketConnection => None,
    };
    KodexResponsesMetadata {
        turn_id: request_kind.and(turn_id.map(ToString::to_string)),
        request_kind,
        parent_thread_id,
        subagent_header: subagent_header_value(session_source),
        subagent_kind: request_kind.and_then(|_| subagent_metadata_kind(session_source)),
        ..KodexResponsesMetadata::new(
            installation_id.to_string(),
            session_id.to_string(),
            thread_id.to_string(),
            window_id,
        )
    }
}

pub fn with_parent_turn(mut metadata: KodexResponsesMetadata, id: &str) -> KodexResponsesMetadata {
    metadata.parent_turn_id = Some(id.to_string());
    metadata
}

pub fn all_model_presets() -> &'static Vec<ModelPreset> {
    &TEST_MODEL_PRESETS
}

pub fn builtin_collaboration_mode_presets() -> Vec<CollaborationModeMask> {
    collaboration_mode_presets::builtin_collaboration_mode_presets()
}
