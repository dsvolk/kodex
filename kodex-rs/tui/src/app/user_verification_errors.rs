//! Safe user-facing messages for typed local verification failures.

use kodex_app_server_client::TypedRequestError;
use kodex_app_server_protocol::UserVerificationCancellationReason;
use kodex_app_server_protocol::UserVerificationErrorDetails;
use kodex_app_server_protocol::UserVerificationFailureReason;
use kodex_app_server_protocol::UserVerificationInvalidRequestReason;
use kodex_app_server_protocol::UserVerificationUnavailableReason;

pub(super) fn verification_error_message(error: &TypedRequestError) -> &'static str {
    let TypedRequestError::Server { source, .. } = error else {
        return "Could not complete user verification with the local Kodex binary.";
    };
    let details = source
        .data
        .clone()
        .and_then(|data| serde_json::from_value::<UserVerificationErrorDetails>(data).ok());
    match details {
        Some(UserVerificationErrorDetails::InvalidRequest {
            reason: UserVerificationInvalidRequestReason::InvalidParams,
        }) => "The local Kodex binary could not verify this request.",
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::CredentialMissing,
        }) => "No user-verification credential is available in the local Kodex binary.",
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::BiometricsUnavailable,
        }) => "Biometric verification is currently unavailable on this device.",
        Some(UserVerificationErrorDetails::Unavailable {
            reason: UserVerificationUnavailableReason::ProviderUnavailable,
        }) => "User verification is unavailable for this request.",
        Some(UserVerificationErrorDetails::Cancelled {
            reason: UserVerificationCancellationReason::UserCancelled,
        }) => "User verification was cancelled.",
        Some(UserVerificationErrorDetails::Cancelled {
            reason: UserVerificationCancellationReason::Interrupted,
        }) => "User verification was interrupted.",
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::AuthenticationFailed,
        }) => "Biometric verification did not succeed. The request was cancelled.",
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::Timeout,
        }) => "User verification timed out. The request was cancelled.",
        Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::ProviderError,
        })
        | Some(UserVerificationErrorDetails::Failed {
            reason: UserVerificationFailureReason::ServiceError,
        })
        | None => "The local Kodex binary could not complete user verification.",
    }
}

#[cfg(test)]
#[path = "user_verification_errors_tests.rs"]
mod tests;
