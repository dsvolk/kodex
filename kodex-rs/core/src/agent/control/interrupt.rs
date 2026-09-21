//! Applies V2 interruption rules to a registered agent without loading its runtime.
//!
//! Root and self targets are rejected. An unloaded or already-dead runtime is a successful
//! interruption; this operation never reloads it.

use super::LocalAgentControl;
use crate::agent::api::AgentInfo;
use crate::agent::api::AgentTarget;
use kodex_protocol::AgentPath;
use kodex_protocol::ThreadId;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::KodexErrorDetails;
use kodex_protocol::error::Result as KodexResult;

impl LocalAgentControl {
    /// Interrupts a spawned agent's current task, preserving the status observed before dispatch.
    pub(crate) async fn interrupt_spawned_agent(
        &self,
        caller: ThreadId,
        target: ThreadId,
    ) -> KodexResult<AgentInfo> {
        let receiver_agent = self.ensure_agent_known(target)?;
        if receiver_agent
            .agent_path
            .as_ref()
            .is_some_and(AgentPath::is_root)
        {
            return Err(KodexErr::UnsupportedOperation(
                "root is not a spawned agent".to_string(),
            ));
        }
        if target == caller {
            return Err(KodexErr::UnsupportedOperation(
                "an agent cannot interrupt itself; return your result and let the parent interrupt you if needed"
                    .to_string(),
            ));
        }
        receiver_agent.agent_path.as_ref().ok_or_else(|| {
            KodexErr::UnsupportedOperation("target agent is missing an agent_path".to_string())
        })?;
        let snapshot = self.inspect(caller, AgentTarget::Id(target)).await?;
        match self.interrupt_agent(target).await {
            Ok(_) => {}
            Err(err)
                if matches!(
                    err.details(),
                    KodexErrorDetails::ThreadNotFound(_) | KodexErrorDetails::InternalAgentDied
                ) => {}
            Err(err) => return Err(err),
        }
        Ok(snapshot)
    }
}
