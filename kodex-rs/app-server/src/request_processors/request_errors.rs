use super::*;
use kodex_protocol::error::KodexErrorDetails;

pub(super) fn environment_selection_error(err: KodexErr) -> JSONRPCErrorError {
    match err.details() {
        KodexErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(format!("failed to validate environment selections: {err}")),
    }
}
