pub const TOOL_CALL_COUNT_METRIC: &str = "kodex.tool.call";
pub const TOOL_CALL_DURATION_METRIC: &str = "kodex.tool.call.duration_ms";
pub const TOOL_CALL_UNIFIED_EXEC_METRIC: &str = "kodex.tool.unified_exec";
pub const ARTIFACT_OPERATION_STARTED_METRIC: &str = "kodex.artifact.operation.started";
pub const ARTIFACT_OPERATION_EXPECTED_OUTPUT_COUNT_METRIC: &str =
    "kodex.artifact.operation.expected_output_count";
pub const PROCESS_START_METRIC: &str = "kodex.process.start";
/// Caller-side exec-server RPC attempts, including local admission and transport failures.
pub const EXEC_SERVER_CLIENT_REQUEST_COUNT_METRIC: &str = "exec_server_client_requests_total";
pub const API_CALL_COUNT_METRIC: &str = "kodex.api_request";
pub const API_CALL_DURATION_METRIC: &str = "kodex.api_request.duration_ms";
pub const SSE_EVENT_COUNT_METRIC: &str = "kodex.sse_event";
pub const SSE_EVENT_DURATION_METRIC: &str = "kodex.sse_event.duration_ms";
pub const WEBSOCKET_REQUEST_COUNT_METRIC: &str = "kodex.websocket.request";
pub const WEBSOCKET_CONTINUATION_COUNT_METRIC: &str = "kodex.websocket.continuation";
pub const WEBSOCKET_REQUEST_DURATION_METRIC: &str = "kodex.websocket.request.duration_ms";
pub const WEBSOCKET_EVENT_COUNT_METRIC: &str = "kodex.websocket.event";
pub const WEBSOCKET_EVENT_DURATION_METRIC: &str = "kodex.websocket.event.duration_ms";
pub const RESPONSES_API_OVERHEAD_DURATION_METRIC: &str = "kodex.responses_api_overhead.duration_ms";
pub const RESPONSES_API_INFERENCE_TIME_DURATION_METRIC: &str =
    "kodex.responses_api_inference_time.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TTFT_DURATION_METRIC: &str =
    "kodex.responses_api_engine_iapi_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TTFT_DURATION_METRIC: &str =
    "kodex.responses_api_engine_service_ttft.duration_ms";
pub const RESPONSES_API_ENGINE_IAPI_TBT_DURATION_METRIC: &str =
    "kodex.responses_api_engine_iapi_tbt.duration_ms";
pub const RESPONSES_API_ENGINE_SERVICE_TBT_DURATION_METRIC: &str =
    "kodex.responses_api_engine_service_tbt.duration_ms";
pub const TURN_E2E_DURATION_METRIC: &str = "kodex.turn.e2e_duration_ms";
pub const TURN_TTFT_DURATION_METRIC: &str = "kodex.turn.ttft.duration_ms";
pub const TURN_TTFM_DURATION_METRIC: &str = "kodex.turn.ttfm.duration_ms";
pub const TURN_NETWORK_PROXY_METRIC: &str = "kodex.turn.network_proxy";
pub const TURN_MEMORY_METRIC: &str = "kodex.turn.memory";
pub const TURN_TOOL_CALL_METRIC: &str = "kodex.turn.tool.call";
pub const TURN_TOKEN_USAGE_METRIC: &str = "kodex.turn.token_usage";
pub const TURN_COST_MICROUSD_METRIC: &str = "kodex.turn.cost_microusd";
pub const TURN_UNIFIED_EXEC_RUNNING_PROCESSES_METRIC: &str =
    "kodex.turn.unified_exec.running_processes";
pub const GUARDIAN_REVIEW_COUNT_METRIC: &str = "kodex.guardian.review";
pub const GUARDIAN_REVIEW_DURATION_METRIC: &str = "kodex.guardian.review.duration_ms";
pub const GUARDIAN_REVIEW_TTFT_DURATION_METRIC: &str = "kodex.guardian.review.ttft.duration_ms";
pub const GUARDIAN_REVIEW_TOKEN_USAGE_METRIC: &str = "kodex.guardian.review.token_usage";
pub const GOAL_CREATED_METRIC: &str = "kodex.goal.created";
pub const GOAL_RESUMED_METRIC: &str = "kodex.goal.resumed";
pub const GOAL_COMPLETED_METRIC: &str = "kodex.goal.completed";
pub const GOAL_BUDGET_LIMITED_METRIC: &str = "kodex.goal.budget_limited";
pub const GOAL_USAGE_LIMITED_METRIC: &str = "kodex.goal.usage_limited";
pub const GOAL_BLOCKED_METRIC: &str = "kodex.goal.blocked";
pub const GOAL_TOKEN_COUNT_METRIC: &str = "kodex.goal.token_count";
pub const GOAL_DURATION_SECONDS_METRIC: &str = "kodex.goal.duration_s";
pub const PLUGIN_INSTALL_ELICITATION_SENT_METRIC: &str = "kodex.plugins.install_elicitation.sent";
pub const PLUGIN_INSTALL_SUGGESTION_METRIC: &str = "kodex.plugins.install_suggestion";
pub const CURATED_PLUGINS_STARTUP_SYNC_METRIC: &str = "kodex.plugins.startup_sync";
pub const CURATED_PLUGINS_STARTUP_SYNC_FINAL_METRIC: &str = "kodex.plugins.startup_sync.final";
pub const HOOK_RUN_METRIC: &str = "kodex.hooks.run";
pub const HOOK_RUN_DURATION_METRIC: &str = "kodex.hooks.run.duration_ms";
/// Duration for coarse startup phases, tagged by low-cardinality phase and status.
pub const STARTUP_PHASE_DURATION_METRIC: &str = "kodex.startup.phase.duration_ms";
/// Total runtime of a startup prewarm attempt until it completes, tagged by final status.
pub const STARTUP_PREWARM_DURATION_METRIC: &str = "kodex.startup_prewarm.duration_ms";
/// Age of the startup prewarm attempt when the first real turn resolves it, tagged by outcome.
pub const STARTUP_PREWARM_AGE_AT_FIRST_TURN_METRIC: &str =
    "kodex.startup_prewarm.age_at_first_turn_ms";
pub const THREAD_STARTED_METRIC: &str = "kodex.thread.started";
pub const THREAD_SKILLS_ENABLED_TOTAL_METRIC: &str = "kodex.thread.skills.enabled_total";
pub const THREAD_SKILLS_KEPT_TOTAL_METRIC: &str = "kodex.thread.skills.kept_total";
pub const THREAD_SKILLS_DESCRIPTION_TRUNCATED_CHARS_METRIC: &str =
    "kodex.thread.skills.description_truncated_chars";
pub const THREAD_SKILLS_TRUNCATED_METRIC: &str = "kodex.thread.skills.truncated";
