//! Binds cached classifier results to the user authorization and model policy they evaluated.

use kodex_core::GuardianAuthorizationVersion;
use kodex_core::KodexThread;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ScoreAuthorization {
    pub(super) settings: kodex_protocol::protocol::ThreadSettingsSnapshot,
    pub(super) environments: Vec<kodex_protocol::protocol::TurnEnvironmentSelection>,
    pub(super) local: GuardianAuthorizationVersion,
    pub(super) root: Option<GuardianAuthorizationVersion>,
    pub(super) model: Option<std::sync::Arc<kodex_protocol::openai_models::ModelInfo>>,
}

impl ScoreAuthorization {
    pub(super) async fn current(thread: &KodexThread) -> Self {
        let root = thread
            .guardian_root_snapshot()
            .await
            .map(|snapshot| snapshot.authorization_version);
        Self {
            settings: thread.thread_settings_snapshot().await,
            environments: thread
                .config_snapshot()
                .await
                .environment_selections()
                .to_vec(),
            local: thread.guardian_authorization_version().await,
            root,
            model: thread
                .thread_extension_data()
                .get::<kodex_protocol::openai_models::ModelInfo>(),
        }
    }
}
