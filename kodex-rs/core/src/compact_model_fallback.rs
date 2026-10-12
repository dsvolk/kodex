use crate::session::turn_context::TurnContext;
use kodex_analytics::CompactionImplementation;
use kodex_analytics::CompactionReason;
use kodex_otel::SessionTelemetry;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::KodexErrorDetails;
use tracing::warn;

/// Returns whether a failed compaction attempt should use the current model.
pub(crate) fn should_retry_with_current_model(
    error: &KodexErr,
    previous: &TurnContext,
    current: &TurnContext,
) -> bool {
    if matches!(
        error.details(),
        KodexErrorDetails::TurnAborted
            | KodexErrorDetails::Interrupted
            | KodexErrorDetails::SessionBudgetExceeded
    ) || (previous.model_info().slug == current.model_info().slug
        && previous.cyber_access_program == current.cyber_access_program)
    {
        return false;
    }

    current.provider.info().is_openai()
        && current
            .auth_manager
            .as_deref()
            .and_then(kodex_login::AuthManager::auth_cached)
            .is_some_and(|auth| auth.uses_kodex_backend() || auth.is_api_key_auth())
}

pub(crate) fn record_model_fallback(
    session_telemetry: &SessionTelemetry,
    previous_model: &str,
    current_model: &str,
    reason: CompactionReason,
    implementation: CompactionImplementation,
    fallback_error: Option<&KodexErr>,
) {
    let reason_tag = match reason {
        CompactionReason::UserRequested => "user_requested",
        CompactionReason::ContextLimit => "context_limit",
        CompactionReason::ModelDownshift => "model_downshift",
        CompactionReason::CompHashChanged => "comp_hash_changed",
    };
    let implementation_tag = match implementation {
        CompactionImplementation::Responses => "responses",
        CompactionImplementation::ResponsesCompactionV2 => "responses_compaction_v2",
    };
    let outcome = if fallback_error.is_none() {
        "succeeded"
    } else {
        "failed"
    };
    session_telemetry.counter(
        "kodex.compaction.model_fallback",
        /*inc*/ 1,
        &[
            ("reason", reason_tag),
            ("implementation", implementation_tag),
            ("outcome", outcome),
        ],
    );
    warn!(
        previous_model,
        current_model,
        ?reason,
        ?implementation,
        outcome,
        ?fallback_error,
        "previous-model compaction failed; retried with current model"
    );
}
