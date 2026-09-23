//! Installs caller-bound tools using persistent host identities.
//!
//! Backend selection is a host concern. Startup failures leave tools unavailable
//! and emit a warning; a disabled factory does not open storage.
//! Tool namespaces follow the host's configuration at runtime startup.

use crate::AgentMessageBoard;
use crate::message_board_tools;
use futures::future::BoxFuture;
use kodex_extension_api::ExtensionData;
use kodex_extension_api::ExtensionEventSink;
use kodex_extension_api::ExtensionFuture;
use kodex_extension_api::ExtensionRegistryBuilder;
use kodex_extension_api::ExtensionWarning;
use kodex_extension_api::ThreadLifecycleContributor;
use kodex_extension_api::ThreadStartInput;
use kodex_extension_api::ToolContributor;
use kodex_protocol::AgentPath;
use kodex_protocol::SessionId;
use kodex_protocol::ThreadId;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::Result;
use kodex_tools::ToolCall;
use kodex_tools::ToolExecutor;
use std::sync::Arc;

type BoardFactory<C> = dyn Fn(&C, SessionId, ThreadId) -> BoxFuture<'static, Result<Option<Arc<dyn AgentMessageBoard>>>>
    + Send
    + Sync;
type NamespaceResolver<C> = dyn Fn(&C) -> Option<String> + Send + Sync;

struct BoardExtension<C> {
    open: Box<BoardFactory<C>>,
    tool_namespace: Box<NamespaceResolver<C>>,
    namespace_description: &'static str,
    events: Arc<dyn ExtensionEventSink>,
}
struct Binding {
    board: Arc<dyn AgentMessageBoard>,
    caller: ThreadId,
    path: AgentPath,
    namespace: Option<String>,
}

impl<C: Sync> ThreadLifecycleContributor<C> for BoardExtension<C> {
    fn on_thread_start<'a>(&'a self, input: ThreadStartInput<'a, C>) -> ExtensionFuture<'a, ()> {
        Box::pin(async move {
            let result = async {
                let tree =
                    SessionId::from_string(input.session_store.level_id()).map_err(|_| {
                        KodexErr::InvalidRequest("invalid board session identity".into())
                    })?;
                let caller =
                    ThreadId::from_string(input.thread_store.level_id()).map_err(|_| {
                        KodexErr::InvalidRequest("invalid board caller identity".into())
                    })?;
                if let Some(board) = (self.open)(input.config, tree, caller).await? {
                    if board.identity() != tree {
                        return Err(KodexErr::InvalidRequest(
                            "message-board factory returned another tree".into(),
                        ));
                    }
                    input.thread_store.insert(Binding {
                        board,
                        caller,
                        namespace: (self.tool_namespace)(input.config),
                        path: input
                            .session_source
                            .get_agent_path()
                            .unwrap_or_else(AgentPath::root),
                    });
                }
                Ok::<(), KodexErr>(())
            }
            .await;
            if result.is_err() {
                self.events.emit_warning(ExtensionWarning {
                    thread_id: input.thread_store.level_id().into(), turn_id: None,
                    message: "Agent message-board initialization failed; its tools are unavailable for this runtime.".into(),
                });
            }
        })
    }
}

impl<C: Sync> ToolContributor for BoardExtension<C> {
    fn tools(
        &self,
        _session_store: &ExtensionData,
        thread_store: &ExtensionData,
    ) -> Vec<Arc<dyn for<'call> ToolExecutor<ToolCall<'call>>>> {
        thread_store
            .get::<Binding>()
            .map_or_else(Vec::new, |binding| {
                message_board_tools(
                    binding.board.clone(),
                    binding.caller,
                    binding.path.clone(),
                    binding.namespace.as_deref(),
                    self.namespace_description,
                )
            })
    }
}

/// Installs message-board lifecycle and tool contributions. The factory selects
/// a local or remote backend, or returns None when disabled. Configuration is
/// read at runtime startup, including resume; no board is created by installation.
/// The host supplies its shared namespace description and resolves the namespace
/// name from the runtime's startup configuration.
pub fn install<C: Sync + 'static>(
    registry: &mut ExtensionRegistryBuilder<C>,
    namespace_description: &'static str,
    tool_namespace: impl Fn(&C) -> Option<String> + Send + Sync + 'static,
    open: impl Fn(
        &C,
        SessionId,
        ThreadId,
    ) -> BoxFuture<'static, Result<Option<Arc<dyn AgentMessageBoard>>>>
    + Send
    + Sync
    + 'static,
) {
    let extension = Arc::new(BoardExtension {
        open: Box::new(open),
        tool_namespace: Box::new(tool_namespace),
        namespace_description,
        events: registry.event_sink(),
    });
    registry.thread_lifecycle_contributor(extension.clone());
    registry.tool_contributor(extension);
}
