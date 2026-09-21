//! Resolves controller targets using the caller's registered identity.
//! Legacy callers can still supply their captured session source for path resolution.

use super::LocalAgentControl;
use crate::agent::api::AgentTarget;
use kodex_protocol::AgentPath;
use kodex_protocol::ThreadId;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::Result as KodexResult;
use kodex_protocol::protocol::SessionSource;

impl LocalAgentControl {
    pub(crate) fn resolve_target(
        &self,
        caller: ThreadId,
        target: &AgentTarget,
    ) -> KodexResult<ThreadId> {
        match target {
            AgentTarget::Id(thread_id) => Ok(*thread_id),
            AgentTarget::Reference(reference) => {
                let caller = self.ensure_agent_known(caller)?;
                self.resolve_path_reference(
                    &caller.agent_path.unwrap_or_else(AgentPath::root),
                    reference,
                )
            }
        }
    }

    pub(crate) async fn resolve_agent_reference(
        &self,
        _current_thread_id: ThreadId,
        current_session_source: &SessionSource,
        agent_reference: &str,
    ) -> KodexResult<ThreadId> {
        let current_agent_path = current_session_source
            .get_agent_path()
            .unwrap_or_else(AgentPath::root);
        self.resolve_path_reference(&current_agent_path, agent_reference)
    }

    fn resolve_path_reference(
        &self,
        current_agent_path: &AgentPath,
        agent_reference: &str,
    ) -> KodexResult<ThreadId> {
        let agent_path = current_agent_path
            .resolve(agent_reference)
            .map_err(KodexErr::UnsupportedOperation)?;
        if let Some(thread_id) = self.state.agent_id_for_path(&agent_path) {
            return Ok(thread_id);
        }
        Err(KodexErr::UnsupportedOperation(format!(
            "live agent path `{}` not found",
            agent_path.as_str()
        )))
    }
}
