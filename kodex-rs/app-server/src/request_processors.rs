use crate::bespoke_event_handling::apply_bespoke_event_handling;
use crate::command_exec::CommandExecManager;
use crate::command_exec::StartCommandExecParams;
use crate::config_manager::ConfigManager;
use crate::error_code::INPUT_TOO_LARGE_ERROR_CODE;
use crate::error_code::invalid_params;
use crate::models::supported_models;
use crate::outgoing_message::ConnectionId;
use crate::outgoing_message::ConnectionRequestId;
use crate::outgoing_message::OutgoingMessageSender;
use crate::outgoing_message::RequestContext;
use crate::outgoing_message::ThreadScopedOutgoingMessageSender;
use crate::skills_watcher::SkillsWatcher;
use crate::thread_status::ThreadWatchManager;
use crate::thread_status::resolve_thread_status;
use chrono::Duration as ChronoDuration;
use chrono::SecondsFormat;
use kodex_analytics::AnalyticsEventsClient;
use kodex_analytics::AnalyticsJsonRpcError;
use kodex_analytics::InputError;
use kodex_analytics::TurnSteerRequestError;
use kodex_app_server_protocol::Account;
use kodex_app_server_protocol::AccountLoginCompletedNotification;
use kodex_app_server_protocol::AccountTokenUsageDailyBucket;
use kodex_app_server_protocol::AccountTokenUsageSummary;
use kodex_app_server_protocol::AccountUpdatedNotification;
use kodex_app_server_protocol::AddCreditsNudgeCreditType;
use kodex_app_server_protocol::AddCreditsNudgeEmailStatus;
use kodex_app_server_protocol::AdditionalContextEntry;
use kodex_app_server_protocol::AdditionalContextKind;
use kodex_app_server_protocol::AppListUpdatedNotification;
use kodex_app_server_protocol::AppSummary;
use kodex_app_server_protocol::AppTemplateSummary;
use kodex_app_server_protocol::AppTemplateUnavailableReason;
use kodex_app_server_protocol::AppsInstalledParams;
use kodex_app_server_protocol::AppsInstalledResponse;
use kodex_app_server_protocol::AppsListParams;
use kodex_app_server_protocol::AppsListResponse;
use kodex_app_server_protocol::AppsReadParams;
use kodex_app_server_protocol::AppsReadResponse;
use kodex_app_server_protocol::AskForApproval;
use kodex_app_server_protocol::AuthMode;
use kodex_app_server_protocol::CancelLoginAccountParams;
use kodex_app_server_protocol::CancelLoginAccountResponse;
use kodex_app_server_protocol::CancelLoginAccountStatus;
use kodex_app_server_protocol::ClientInfo;
use kodex_app_server_protocol::ClientRequest;
use kodex_app_server_protocol::ClientResponsePayload;
use kodex_app_server_protocol::CollaborationModeListParams;
use kodex_app_server_protocol::CollaborationModeListResponse;
use kodex_app_server_protocol::CommandExecParams;
use kodex_app_server_protocol::CommandExecResizeParams;
use kodex_app_server_protocol::CommandExecTerminateParams;
use kodex_app_server_protocol::CommandExecWriteParams;
use kodex_app_server_protocol::ConfigWarningNotification;
use kodex_app_server_protocol::ConsumeAccountRateLimitResetCreditOutcome;
use kodex_app_server_protocol::ConsumeAccountRateLimitResetCreditParams;
use kodex_app_server_protocol::ConsumeAccountRateLimitResetCreditResponse;
use kodex_app_server_protocol::ConversationGitInfo;
use kodex_app_server_protocol::ConversationSummary;
use kodex_app_server_protocol::DeprecationNoticeNotification;
use kodex_app_server_protocol::DynamicToolFunctionSpec;
use kodex_app_server_protocol::DynamicToolNamespaceTool;
use kodex_app_server_protocol::DynamicToolSpec;
use kodex_app_server_protocol::EnvironmentAddParams;
use kodex_app_server_protocol::EnvironmentAddResponse;
use kodex_app_server_protocol::EnvironmentInfoParams;
use kodex_app_server_protocol::EnvironmentInfoResponse;
use kodex_app_server_protocol::EnvironmentShellInfo;
use kodex_app_server_protocol::EnvironmentStatusKind;
use kodex_app_server_protocol::EnvironmentStatusParams;
use kodex_app_server_protocol::EnvironmentStatusResponse;
use kodex_app_server_protocol::ExperimentalFeature as ApiExperimentalFeature;
use kodex_app_server_protocol::ExperimentalFeatureListParams;
use kodex_app_server_protocol::ExperimentalFeatureListResponse;
use kodex_app_server_protocol::ExperimentalFeatureStage as ApiExperimentalFeatureStage;
use kodex_app_server_protocol::FeedbackUploadParams;
use kodex_app_server_protocol::FeedbackUploadResponse;
use kodex_app_server_protocol::GetAccountParams;
use kodex_app_server_protocol::GetAccountRateLimitsResponse;
use kodex_app_server_protocol::GetAccountResponse;
use kodex_app_server_protocol::GetAccountTokenUsageParams;
use kodex_app_server_protocol::GetAccountTokenUsageResponse;
use kodex_app_server_protocol::GetAuthStatusParams;
use kodex_app_server_protocol::GetAuthStatusResponse;
use kodex_app_server_protocol::GetConversationSummaryParams;
use kodex_app_server_protocol::GetConversationSummaryResponse;
use kodex_app_server_protocol::GetWorkspaceMessagesResponse;
use kodex_app_server_protocol::GitDiffToRemoteParams;
use kodex_app_server_protocol::GitDiffToRemoteResponse;
use kodex_app_server_protocol::GitInfo as ApiGitInfo;
use kodex_app_server_protocol::HookHandlerMetadata;
use kodex_app_server_protocol::HookMetadata;
use kodex_app_server_protocol::HooksListParams;
use kodex_app_server_protocol::HooksListResponse;
use kodex_app_server_protocol::InitializeParams;
use kodex_app_server_protocol::InitializeResponse;
use kodex_app_server_protocol::InstalledApp;
use kodex_app_server_protocol::JSONRPCErrorError;
use kodex_app_server_protocol::KodexErrorInfo;
use kodex_app_server_protocol::ListMcpServerStatusParams;
use kodex_app_server_protocol::ListMcpServerStatusResponse;
use kodex_app_server_protocol::LoginAccountParams;
use kodex_app_server_protocol::LoginAccountResponse;
use kodex_app_server_protocol::LoginApiKeyParams;
use kodex_app_server_protocol::LoginAppBrand;
use kodex_app_server_protocol::LogoutAccountResponse;
use kodex_app_server_protocol::MarketplaceAddParams;
use kodex_app_server_protocol::MarketplaceAddResponse;
use kodex_app_server_protocol::MarketplaceInterface;
use kodex_app_server_protocol::MarketplaceRemoveParams;
use kodex_app_server_protocol::MarketplaceRemoveResponse;
use kodex_app_server_protocol::MarketplaceUpgradeErrorInfo;
use kodex_app_server_protocol::MarketplaceUpgradeParams;
use kodex_app_server_protocol::MarketplaceUpgradeResponse;
use kodex_app_server_protocol::McpResourceReadParams;
use kodex_app_server_protocol::McpResourceReadResponse;
use kodex_app_server_protocol::McpServerOauthClientRegistration;
use kodex_app_server_protocol::McpServerOauthLoginCompletedNotification;
use kodex_app_server_protocol::McpServerOauthLoginParams;
use kodex_app_server_protocol::McpServerOauthLoginResponse;
use kodex_app_server_protocol::McpServerRefreshResponse;
use kodex_app_server_protocol::McpServerStatus;
use kodex_app_server_protocol::McpServerStatusDetail;
use kodex_app_server_protocol::McpServerToolCallParams;
use kodex_app_server_protocol::McpServerToolCallResponse;
use kodex_app_server_protocol::MemoryResetResponse;
use kodex_app_server_protocol::MockExperimentalMethodParams;
use kodex_app_server_protocol::MockExperimentalMethodResponse;
use kodex_app_server_protocol::ModelListParams;
use kodex_app_server_protocol::ModelListResponse;
use kodex_app_server_protocol::PermissionProfileListParams;
use kodex_app_server_protocol::PermissionProfileListResponse;
use kodex_app_server_protocol::PermissionProfileSummary;
use kodex_app_server_protocol::PluginDetail;
use kodex_app_server_protocol::PluginInstallParams;
use kodex_app_server_protocol::PluginInstallResponse;
use kodex_app_server_protocol::PluginInstalledParams;
use kodex_app_server_protocol::PluginInstalledResponse;
use kodex_app_server_protocol::PluginInterface;
use kodex_app_server_protocol::PluginListMarketplaceKind;
use kodex_app_server_protocol::PluginListParams;
use kodex_app_server_protocol::PluginListResponse;
use kodex_app_server_protocol::PluginMarketplaceEntry;
use kodex_app_server_protocol::PluginReadParams;
use kodex_app_server_protocol::PluginReadResponse;
use kodex_app_server_protocol::PluginShareCheckoutParams;
use kodex_app_server_protocol::PluginShareCheckoutResponse;
use kodex_app_server_protocol::PluginShareContext;
use kodex_app_server_protocol::PluginShareDeleteParams;
use kodex_app_server_protocol::PluginShareDeleteResponse;
use kodex_app_server_protocol::PluginShareDiscoverability;
use kodex_app_server_protocol::PluginShareListItem;
use kodex_app_server_protocol::PluginShareListParams;
use kodex_app_server_protocol::PluginShareListResponse;
use kodex_app_server_protocol::PluginSharePrincipal;
use kodex_app_server_protocol::PluginSharePrincipalType;
use kodex_app_server_protocol::PluginShareSaveParams;
use kodex_app_server_protocol::PluginShareSaveResponse;
use kodex_app_server_protocol::PluginShareTarget;
use kodex_app_server_protocol::PluginShareUpdateDiscoverability;
use kodex_app_server_protocol::PluginShareUpdateTargetsParams;
use kodex_app_server_protocol::PluginShareUpdateTargetsResponse;
use kodex_app_server_protocol::PluginSkillReadParams;
use kodex_app_server_protocol::PluginSkillReadResponse;
use kodex_app_server_protocol::PluginSource;
use kodex_app_server_protocol::PluginSummary;
use kodex_app_server_protocol::PluginUninstallParams;
use kodex_app_server_protocol::PluginUninstallResponse;
use kodex_app_server_protocol::RateLimitResetCredit;
use kodex_app_server_protocol::RateLimitResetCreditStatus;
use kodex_app_server_protocol::RateLimitResetCreditsSummary;
use kodex_app_server_protocol::RateLimitResetType;
use kodex_app_server_protocol::RequestId;
use kodex_app_server_protocol::ReviewDelivery as ApiReviewDelivery;
use kodex_app_server_protocol::ReviewStartParams;
use kodex_app_server_protocol::ReviewStartResponse;
use kodex_app_server_protocol::ReviewTarget as ApiReviewTarget;
use kodex_app_server_protocol::SandboxMode;
use kodex_app_server_protocol::SendAddCreditsNudgeEmailParams;
use kodex_app_server_protocol::SendAddCreditsNudgeEmailResponse;
use kodex_app_server_protocol::ServerNotification;
use kodex_app_server_protocol::ServerRequestResolvedNotification;
use kodex_app_server_protocol::SkillSummary;
use kodex_app_server_protocol::SkillsConfigWriteParams;
use kodex_app_server_protocol::SkillsConfigWriteResponse;
use kodex_app_server_protocol::SkillsExtraRootsSetParams;
use kodex_app_server_protocol::SkillsExtraRootsSetResponse;
use kodex_app_server_protocol::SkillsListParams;
use kodex_app_server_protocol::SkillsListResponse;
use kodex_app_server_protocol::SortDirection;
use kodex_app_server_protocol::Thread;
use kodex_app_server_protocol::ThreadApproveGuardianDeniedActionParams;
use kodex_app_server_protocol::ThreadApproveGuardianDeniedActionResponse;
use kodex_app_server_protocol::ThreadArchiveParams;
use kodex_app_server_protocol::ThreadArchiveResponse;
use kodex_app_server_protocol::ThreadArchivedNotification;
use kodex_app_server_protocol::ThreadBackgroundTerminal;
use kodex_app_server_protocol::ThreadBackgroundTerminalsCleanParams;
use kodex_app_server_protocol::ThreadBackgroundTerminalsCleanResponse;
use kodex_app_server_protocol::ThreadBackgroundTerminalsListParams;
use kodex_app_server_protocol::ThreadBackgroundTerminalsListResponse;
use kodex_app_server_protocol::ThreadBackgroundTerminalsTerminateParams;
use kodex_app_server_protocol::ThreadBackgroundTerminalsTerminateResponse;
use kodex_app_server_protocol::ThreadClosedNotification;
use kodex_app_server_protocol::ThreadCompactStartParams;
use kodex_app_server_protocol::ThreadCompactStartResponse;
use kodex_app_server_protocol::ThreadDecrementElicitationParams;
use kodex_app_server_protocol::ThreadDecrementElicitationResponse;
use kodex_app_server_protocol::ThreadDeleteParams;
use kodex_app_server_protocol::ThreadDeleteResponse;
use kodex_app_server_protocol::ThreadDeletedNotification;
use kodex_app_server_protocol::ThreadForkParams;
use kodex_app_server_protocol::ThreadForkResponse;
use kodex_app_server_protocol::ThreadGoal;
use kodex_app_server_protocol::ThreadGoalClearParams;
use kodex_app_server_protocol::ThreadGoalClearResponse;
use kodex_app_server_protocol::ThreadGoalClearedNotification;
use kodex_app_server_protocol::ThreadGoalGetParams;
use kodex_app_server_protocol::ThreadGoalGetResponse;
use kodex_app_server_protocol::ThreadGoalSetParams;
use kodex_app_server_protocol::ThreadGoalSetResponse;
use kodex_app_server_protocol::ThreadGoalStatus;
use kodex_app_server_protocol::ThreadGoalUpdatedNotification;
use kodex_app_server_protocol::ThreadHistoryBuilder;
#[cfg(test)]
use kodex_app_server_protocol::ThreadHistoryMode;
use kodex_app_server_protocol::ThreadIncrementElicitationParams;
use kodex_app_server_protocol::ThreadIncrementElicitationResponse;
use kodex_app_server_protocol::ThreadInjectItemsParams;
use kodex_app_server_protocol::ThreadInjectItemsResponse;
use kodex_app_server_protocol::ThreadItem;
use kodex_app_server_protocol::ThreadItemEntry;
use kodex_app_server_protocol::ThreadItemsListParams;
use kodex_app_server_protocol::ThreadItemsListResponse;
use kodex_app_server_protocol::ThreadListCwdFilter;
use kodex_app_server_protocol::ThreadListParams;
use kodex_app_server_protocol::ThreadListResponse;
use kodex_app_server_protocol::ThreadLoadedListParams;
use kodex_app_server_protocol::ThreadLoadedListResponse;
use kodex_app_server_protocol::ThreadMemoryModeSetParams;
use kodex_app_server_protocol::ThreadMemoryModeSetResponse;
use kodex_app_server_protocol::ThreadMetadataGitInfoUpdateParams;
use kodex_app_server_protocol::ThreadMetadataUpdateParams;
use kodex_app_server_protocol::ThreadMetadataUpdateResponse;
use kodex_app_server_protocol::ThreadNameUpdatedNotification;
use kodex_app_server_protocol::ThreadProjectUpdatedNotification;
use kodex_app_server_protocol::ThreadReadParams;
use kodex_app_server_protocol::ThreadReadResponse;
use kodex_app_server_protocol::ThreadRealtimeAppendAudioParams;
use kodex_app_server_protocol::ThreadRealtimeAppendAudioResponse;
use kodex_app_server_protocol::ThreadRealtimeAppendSpeechParams;
use kodex_app_server_protocol::ThreadRealtimeAppendSpeechResponse;
use kodex_app_server_protocol::ThreadRealtimeAppendTextParams;
use kodex_app_server_protocol::ThreadRealtimeAppendTextResponse;
use kodex_app_server_protocol::ThreadRealtimeListVoicesResponse;
use kodex_app_server_protocol::ThreadRealtimeStartParams;
use kodex_app_server_protocol::ThreadRealtimeStartResponse;
use kodex_app_server_protocol::ThreadRealtimeStartTransport;
use kodex_app_server_protocol::ThreadRealtimeStopParams;
use kodex_app_server_protocol::ThreadRealtimeStopResponse;
use kodex_app_server_protocol::ThreadResumeInitialTurnsPageParams;
use kodex_app_server_protocol::ThreadResumeParams;
use kodex_app_server_protocol::ThreadResumeResponse;
use kodex_app_server_protocol::ThreadSearchOccurrence;
use kodex_app_server_protocol::ThreadSearchOccurrencesParams;
use kodex_app_server_protocol::ThreadSearchOccurrencesResponse;
use kodex_app_server_protocol::ThreadSearchParams;
use kodex_app_server_protocol::ThreadSearchResponse;
use kodex_app_server_protocol::ThreadSearchResult;
use kodex_app_server_protocol::ThreadSearchSortKey;
use kodex_app_server_protocol::ThreadSearchTextRange;
use kodex_app_server_protocol::ThreadSetNameParams;
use kodex_app_server_protocol::ThreadSetNameResponse;
use kodex_app_server_protocol::ThreadSettings;
use kodex_app_server_protocol::ThreadSettingsUpdateParams;
use kodex_app_server_protocol::ThreadSettingsUpdateResponse;
use kodex_app_server_protocol::ThreadShellCommandParams;
use kodex_app_server_protocol::ThreadShellCommandResponse;
use kodex_app_server_protocol::ThreadSortKey;
use kodex_app_server_protocol::ThreadSourceKind;
use kodex_app_server_protocol::ThreadStartParams;
use kodex_app_server_protocol::ThreadStartResponse;
use kodex_app_server_protocol::ThreadStartedNotification;
use kodex_app_server_protocol::ThreadStatus;
use kodex_app_server_protocol::ThreadTimelineListParams;
use kodex_app_server_protocol::ThreadTimelineListResponse;
use kodex_app_server_protocol::ThreadTurnsListParams;
use kodex_app_server_protocol::ThreadTurnsListResponse;
use kodex_app_server_protocol::ThreadUnarchiveParams;
use kodex_app_server_protocol::ThreadUnarchiveResponse;
use kodex_app_server_protocol::ThreadUnarchivedNotification;
use kodex_app_server_protocol::ThreadUnsubscribeParams;
use kodex_app_server_protocol::ThreadUnsubscribeResponse;
use kodex_app_server_protocol::ThreadUnsubscribeStatus;
use kodex_app_server_protocol::Turn;
use kodex_app_server_protocol::TurnEnvironmentParams;
use kodex_app_server_protocol::TurnError;
use kodex_app_server_protocol::TurnInterruptParams;
use kodex_app_server_protocol::TurnInterruptResponse;
use kodex_app_server_protocol::TurnItemsView;
use kodex_app_server_protocol::TurnSettingsUpdateParams;
use kodex_app_server_protocol::TurnSettingsUpdateResponse;
use kodex_app_server_protocol::TurnSettingsUpdateStatus;
use kodex_app_server_protocol::TurnStartParams;
use kodex_app_server_protocol::TurnStartResponse;
use kodex_app_server_protocol::TurnStatus;
use kodex_app_server_protocol::TurnSteerParams;
use kodex_app_server_protocol::TurnSteerResponse;
use kodex_app_server_protocol::UserInput as V2UserInput;
use kodex_app_server_protocol::WindowsSandboxReadiness;
use kodex_app_server_protocol::WindowsSandboxReadinessResponse;
use kodex_app_server_protocol::WindowsSandboxSetupCompletedNotification;
use kodex_app_server_protocol::WindowsSandboxSetupMode;
use kodex_app_server_protocol::WindowsSandboxSetupStartParams;
use kodex_app_server_protocol::WindowsSandboxSetupStartResponse;
use kodex_app_server_protocol::WorkspaceMessage;
use kodex_app_server_protocol::WorkspaceMessageType;
use kodex_arg0::Arg0DispatchPaths;
use kodex_backend_client::AddCreditsNudgeCreditType as BackendAddCreditsNudgeCreditType;
use kodex_backend_client::Client as BackendClient;
use kodex_backend_client::ConsumeRateLimitResetCreditCode as BackendConsumeRateLimitResetCreditCode;
use kodex_backend_client::KodexWorkspaceMessage as BackendWorkspaceMessage;
use kodex_backend_client::KodexWorkspaceMessageType as BackendWorkspaceMessageType;
use kodex_backend_client::KodexWorkspaceMessagesResponse as BackendWorkspaceMessagesResponse;
use kodex_backend_client::RateLimitResetCreditDetails as BackendRateLimitResetCreditDetails;
use kodex_backend_client::RateLimitResetCreditsDetails as BackendRateLimitResetCreditsDetails;
use kodex_backend_client::RequestError as BackendRequestError;
use kodex_backend_client::TokenUsageProfile;
use kodex_chatgpt::connectors;
use kodex_config::CloudConfigBundleLoadError;
use kodex_config::CloudConfigBundleLoadErrorCode;
use kodex_config::ConfigLayerStack;
use kodex_config::loader::project_trust_key;
use kodex_config::types::McpServerTransportConfig;
use kodex_connectors::AppInfo;
use kodex_core::ForkSnapshot;
use kodex_core::KodexThread;
use kodex_core::KodexThreadSettingsOverrides;
use kodex_core::McpManager;
use kodex_core::NewThread;
use kodex_core::NotSubmittedReason;
#[cfg(test)]
use kodex_core::SessionMeta;
use kodex_core::StartThreadOptions;
use kodex_core::SteerSubmission;
use kodex_core::ThreadConfigSnapshot;
use kodex_core::ThreadManager;
use kodex_core::TurnInput;
use kodex_core::TurnInputRequest;
use kodex_core::TurnInputSubmission;
use kodex_core::TurnStartOptions;
use kodex_core::config::Config;
use kodex_core::config::ConfigOverrides;
use kodex_core::config::NetworkProxyAuditMetadata;
use kodex_core::config::edit::ConfigEdit;
use kodex_core::config::edit::ConfigEditsBuilder;
use kodex_core::connectors::AccessibleConnectorsStatus;
use kodex_core::exec::ExecCapturePolicy;
use kodex_core::exec::ExecExpiration;
use kodex_core::exec::ExecParams;
use kodex_core::exec_env::create_env;
use kodex_core::path_utils;
#[cfg(test)]
use kodex_core::read_head_for_summary;
use kodex_core::sandboxing::SandboxPermissions;
use kodex_core::truncate_rollout_after_turn_id;
use kodex_core::truncate_rollout_before_turn_id;
use kodex_core::windows_sandbox::WindowsSandboxLevelExt;
use kodex_core::windows_sandbox::WindowsSandboxSetupMode as CoreWindowsSandboxSetupMode;
use kodex_core::windows_sandbox::WindowsSandboxSetupRequest;
use kodex_core::windows_sandbox::sandbox_setup_is_complete;
use kodex_core_plugins::PluginInstallError as CorePluginInstallError;
use kodex_core_plugins::PluginInstallRequest;
use kodex_core_plugins::PluginReadRequest;
use kodex_core_plugins::PluginUninstallError as CorePluginUninstallError;
use kodex_core_plugins::PluginsManager;
use kodex_core_plugins::loader::load_plugin_apps;
use kodex_core_plugins::manifest::PluginManifestInterface;
use kodex_core_plugins::marketplace::MarketplaceError;
use kodex_core_plugins::marketplace::MarketplacePluginSource;
use kodex_core_plugins::marketplace_add::MarketplaceAddError;
use kodex_core_plugins::marketplace_add::MarketplaceAddRequest;
use kodex_core_plugins::marketplace_add::add_marketplace as add_marketplace_to_kodex_home;
use kodex_core_plugins::marketplace_remove::MarketplaceRemoveError;
use kodex_core_plugins::marketplace_remove::MarketplaceRemoveRequest as CoreMarketplaceRemoveRequest;
use kodex_core_plugins::marketplace_remove::remove_marketplace;
use kodex_core_plugins::remote::RemoteMarketplace;
use kodex_core_plugins::remote::RemoteMarketplaceSource;
use kodex_core_plugins::remote::RemotePluginCatalogError;
use kodex_core_plugins::remote::RemotePluginDetail as RemoteCatalogPluginDetail;
use kodex_core_plugins::remote::RemotePluginServiceConfig;
use kodex_core_plugins::remote::RemotePluginShareContext as RemoteCatalogPluginShareContext;
use kodex_core_plugins::remote::RemotePluginShareSummary as RemoteCatalogPluginShareSummary;
use kodex_core_plugins::remote::RemotePluginSummary as RemoteCatalogPluginSummary;
use kodex_exec_server::EnvironmentManager;
use kodex_exec_server::EnvironmentObservedStatus;
use kodex_exec_server::LOCAL_ENVIRONMENT_ID;
use kodex_exec_server::LOCAL_FS;
use kodex_features::FEATURES;
use kodex_features::Feature;
use kodex_features::Stage;
use kodex_feedback::FeedbackAttachmentPath;
use kodex_feedback::FeedbackUploadOptions;
use kodex_feedback::KodexFeedback;
use kodex_git_utils::git_diff_to_remote;
use kodex_git_utils::resolve_root_git_project_for_trust;
use kodex_login::AuthManager;
use kodex_login::KODEX_OPEN_APP_URL;
use kodex_login::KodexAuth;
use kodex_login::LoginSuccessPage;
use kodex_login::LoginSuccessPageBrand;
use kodex_login::ServerOptions as LoginServerOptions;
use kodex_login::ShutdownHandle;
use kodex_login::complete_device_code_login;
use kodex_login::login_with_api_key;
use kodex_login::login_with_bedrock_api_key;
use kodex_login::oauth_client_id;
use kodex_login::request_device_code;
use kodex_login::run_login_server;
use kodex_mcp::McpRuntimeContext;
use kodex_mcp::McpServerStatusSnapshot;
use kodex_mcp::McpSnapshotDetail;
use kodex_mcp::collect_mcp_server_status_snapshot_with_detail;
use kodex_mcp::discover_supported_scopes;
use kodex_mcp::read_mcp_resource as read_mcp_resource_without_thread;
use kodex_mcp::resolve_oauth_scopes;
use kodex_memories_write::clear_memory_roots_contents;
use kodex_model_provider::create_model_provider;
use kodex_models_manager::collaboration_mode_presets::builtin_collaboration_mode_presets;
use kodex_protocol::ThreadId;
use kodex_protocol::config_types::CollaborationMode;
use kodex_protocol::config_types::ForcedLoginMethod;
use kodex_protocol::config_types::Personality;
use kodex_protocol::config_types::ReasoningSummary;
use kodex_protocol::config_types::TrustLevel;
use kodex_protocol::config_types::WindowsSandboxLevel;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::Result as KodexResult;
#[cfg(test)]
use kodex_protocol::items::TurnItem;
use kodex_protocol::models::ResponseItem;
use kodex_protocol::openai_models::ReasoningEffort;
use kodex_protocol::protocol::AgentStatus;
use kodex_protocol::protocol::ConversationAudioParams;
use kodex_protocol::protocol::ConversationSpeechParams;
use kodex_protocol::protocol::ConversationStartParams;
use kodex_protocol::protocol::ConversationStartTransport;
use kodex_protocol::protocol::ConversationTextParams;
use kodex_protocol::protocol::EnvironmentConfigState;
use kodex_protocol::protocol::EventMsg;
#[cfg(test)]
use kodex_protocol::protocol::GitInfo as CoreGitInfo;
use kodex_protocol::protocol::McpAuthStatus as CoreMcpAuthStatus;
use kodex_protocol::protocol::Op;
use kodex_protocol::protocol::RealtimeVoicesList;
use kodex_protocol::protocol::ReviewDelivery as CoreReviewDelivery;
use kodex_protocol::protocol::ReviewRequest;
use kodex_protocol::protocol::ReviewTarget as CoreReviewTarget;
use kodex_protocol::protocol::SessionConfiguredEvent;
#[cfg(test)]
use kodex_protocol::protocol::SessionMetaLine;
use kodex_protocol::protocol::TurnEnvironmentSelection;
use kodex_protocol::protocol::TurnEnvironmentSelections;
use kodex_protocol::protocol::W3cTraceContext;
use kodex_protocol::protocol::strip_user_message_prefix;
use kodex_protocol::user_input::MAX_USER_INPUT_TEXT_CHARS;
use kodex_protocol::user_input::UserInput as CoreInputItem;
use kodex_rmcp_client::McpOAuthClientRegistration;
use kodex_rmcp_client::StreamableHttpRedirectMode;
use kodex_rmcp_client::perform_oauth_login_return_url;
use kodex_rollout::InitialHistory;
use kodex_rollout::ResumedHistory;
use kodex_rollout::RolloutItem;
use kodex_rollout::is_persisted_rollout_item;
use kodex_rollout::state_db::StateDbHandle;
use kodex_rollout::state_db::reconcile_rollout;
use kodex_state::ThreadMetadata;
use kodex_state::log_db::LogDbLayer;
use kodex_thread_store::ArchiveThreadParams as StoreArchiveThreadParams;
use kodex_thread_store::ArchiveThreadsParams as StoreArchiveThreadsParams;
use kodex_thread_store::ClearableField as StoreClearableField;
use kodex_thread_store::DeleteThreadsParams as StoreDeleteThreadsParams;
use kodex_thread_store::GitInfoPatch as StoreGitInfoPatch;
use kodex_thread_store::ItemSortKey as StoreItemSortKey;
use kodex_thread_store::ListItemsParams as StoreListItemsParams;
use kodex_thread_store::ListThreadsParams as StoreListThreadsParams;
use kodex_thread_store::ListTimelineParams as StoreListTimelineParams;
use kodex_thread_store::ListTurnsParams as StoreListTurnsParams;
use kodex_thread_store::LoadThreadHistoryParams as StoreLoadThreadHistoryParams;
use kodex_thread_store::LocalThreadStore;
use kodex_thread_store::ReadThreadByRolloutPathParams as StoreReadThreadByRolloutPathParams;
use kodex_thread_store::ReadThreadParams as StoreReadThreadParams;
use kodex_thread_store::SearchThreadOccurrencesParams as StoreSearchThreadOccurrencesParams;
use kodex_thread_store::SearchThreadsParams as StoreSearchThreadsParams;
use kodex_thread_store::SortDirection as StoreSortDirection;
use kodex_thread_store::StoredThread;
use kodex_thread_store::StoredTurn;
use kodex_thread_store::StoredTurnItemsView;
use kodex_thread_store::StoredTurnStatus;
use kodex_thread_store::ThreadMetadataPatch as StoreThreadMetadataPatch;
use kodex_thread_store::ThreadRelationFilter as StoreThreadRelationFilter;
use kodex_thread_store::ThreadSortKey as StoreThreadSortKey;
use kodex_thread_store::ThreadStore;
use kodex_thread_store::ThreadStoreError;
use kodex_utils_absolute_path::AbsolutePathBuf;
use kodex_utils_pty::DEFAULT_OUTPUT_BYTES_CAP;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error as IoError;
use std::path::Path;
use std::path::PathBuf;
use std::result::Result;
use std::sync::Arc;
use std::time::Duration;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::sync::Semaphore;
use tokio::sync::SemaphorePermit;
use tokio::sync::broadcast;
use tokio::sync::oneshot;
use tokio::sync::watch;
use tokio_util::sync::CancellationToken;
use tokio_util::sync::DropGuard;
use tokio_util::task::TaskTracker;
use toml::Value as TomlValue;
use tracing::Instrument;
use tracing::error;
use tracing::info;
use tracing::warn;
use uuid::Uuid;

#[cfg(test)]
use kodex_app_server_protocol::ServerRequest;

mod account_processor;
mod apps_processor;
mod bedrock_auth;
mod catalog_processor;
mod command_exec_processor;
mod config_processor;
mod diagnostics;
mod environment_processor;
mod feedback_doctor_report;
mod feedback_processor;
mod feedback_thread_index;
mod fs_processor;
mod git_processor;
mod initialize_processor;
mod marketplace_processor;
mod mcp_event_stream;
mod mcp_processor;
mod memory_status;
mod persisted_resume_settings;
mod plugins;
mod process_exec_processor;
mod projects;
mod remote_control_processor;
mod rollout;
mod search;
mod thread_attachments;
mod thread_enrichment;
mod thread_fork_goal;
mod thread_input;
mod thread_processor;
mod thread_queue_processor;
mod thread_sections;
mod token_usage_replay;
mod turn_processor;
mod windows_sandbox_processor;

pub(crate) use account_processor::AccountRequestProcessor;
pub(crate) use apps_processor::AppsRequestProcessor;
pub(crate) use catalog_processor::CatalogRequestProcessor;
pub(crate) use command_exec_processor::CommandExecRequestProcessor;
pub(crate) use config_processor::ConfigRequestProcessor;
pub(crate) use diagnostics::read_server_diagnostics;
pub(crate) use environment_processor::EnvironmentRequestProcessor;
pub(crate) use feedback_processor::FeedbackRequestProcessor;
pub(crate) use fs_processor::FsRequestProcessor;
pub(crate) use git_processor::GitRequestProcessor;
pub(crate) use initialize_processor::InitializeRequestProcessor;
pub(crate) use marketplace_processor::MarketplaceRequestProcessor;
pub(crate) use mcp_event_stream::McpEventStreamReady;
pub(crate) use mcp_event_stream::McpEventStreams;
pub(crate) use mcp_processor::McpRequestProcessor;
pub(crate) use plugins::PluginRequestProcessor;
pub(crate) use process_exec_processor::ProcessExecRequestProcessor;
pub(crate) use projects::ProjectRequestProcessor;
pub(crate) use remote_control_processor::RemoteControlRequestProcessor;
pub(crate) use search::SearchRequestProcessor;
pub(crate) use thread_goal_processor::ThreadGoalRequestProcessor;
pub(crate) use thread_processor::ThreadRequestProcessor;
pub(crate) use thread_processor::ThreadResumeTarget;
pub(crate) use thread_queue_processor::ThreadQueueRequestProcessor;
pub(crate) use turn_processor::TurnRequestProcessor;
pub(crate) use windows_sandbox_processor::WindowsSandboxRequestProcessor;

use crate::error_code::internal_error;
use crate::error_code::invalid_request;
use crate::filters::compute_source_filters;
use crate::filters::source_kind_matches;
use crate::thread_state::ConnectionCapabilities;
use crate::thread_state::ThreadListenerCommand;
use crate::thread_state::ThreadState;
use crate::thread_state::ThreadStateManager;
use token_usage_replay::restored_token_usage_turn_id;
use token_usage_replay::send_thread_token_usage_update_to_connection;

pub(crate) fn apply_live_thread_settings(
    thread: &mut Thread,
    config_snapshot: &ThreadConfigSnapshot,
) {
    thread.model = Some(config_snapshot.model.clone());
    thread.reasoning_effort = config_snapshot.reasoning_effort.clone();
    thread.environments = Some(
        config_snapshot
            .environment_selections()
            .iter()
            .map(Into::into)
            .collect(),
    );
}

fn resolve_request_cwd(cwd: Option<PathBuf>) -> Result<Option<AbsolutePathBuf>, JSONRPCErrorError> {
    cwd.map(|cwd| {
        AbsolutePathBuf::relative_to_current_dir(path_utils::normalize_for_native_workdir(cwd))
            .map_err(|err| invalid_request(format!("invalid cwd: {err}")))
    })
    .transpose()
}

fn resolve_turn_environment_selections(
    thread_manager: &ThreadManager,
    environments: Option<Vec<TurnEnvironmentParams>>,
) -> Result<Option<Vec<TurnEnvironmentSelection>>, JSONRPCErrorError> {
    let Some(environments) = environments else {
        return Ok(None);
    };
    let mut selections = Vec::with_capacity(environments.len());
    for environment in environments {
        let environment_id = environment.environment_id;
        let cwd = environment
            .cwd
            .to_inferred_path_uri()
            .ok_or_else(|| {
                invalid_request(format!(
                    "invalid cwd for environment `{environment_id}`: path `{}` does not use absolute POSIX or Windows path syntax",
                    environment.cwd
                ))
            })?;
        let workspace_roots = environment
            .runtime_workspace_roots
            .map(|roots| {
                let mut resolved_roots = Vec::new();
                for root in roots {
                    let root = root.to_inferred_path_uri().ok_or_else(|| {
                        invalid_request(format!(
                            "invalid runtime workspace root for environment `{environment_id}`: path `{root}` does not use absolute POSIX or Windows path syntax"
                        ))
                    })?;
                    if !resolved_roots.contains(&root) {
                        resolved_roots.push(root);
                    }
                }
                Ok::<_, JSONRPCErrorError>(resolved_roots)
            })
            .transpose()?
            .unwrap_or_else(|| vec![cwd.clone()]);
        selections.push(TurnEnvironmentSelection {
            environment_id,
            cwd,
            workspace_roots,
            config: EnvironmentConfigState::FromThread,
        });
    }
    thread_manager
        .validate_environment_selections(&selections)
        .map_err(environment_selection_error)?;
    Ok(Some(selections))
}

fn resolve_runtime_workspace_roots(workspace_roots: Vec<AbsolutePathBuf>) -> Vec<AbsolutePathBuf> {
    let mut resolved_roots = Vec::new();
    for root in workspace_roots {
        if !resolved_roots.iter().any(|existing| existing == &root) {
            resolved_roots.push(root);
        }
    }
    resolved_roots
}

mod config_errors;
mod request_errors;
mod thread_delete;
mod thread_goal_processor;
mod thread_lifecycle;
mod thread_resume_redaction;
mod thread_summary;

use self::config_errors::*;
use self::request_errors::*;
use self::thread_goal_processor::api_thread_goal_from_state;
use self::thread_lifecycle::*;
use self::thread_resume_redaction::*;
use self::thread_summary::*;

pub(crate) use self::thread_lifecycle::populate_thread_turns_from_history;
pub(crate) use self::thread_processor::thread_from_stored_thread;
#[cfg(test)]
pub(crate) use self::thread_summary::read_summary_from_rollout;
#[cfg(test)]
pub(crate) use self::thread_summary::summary_to_thread;
pub(crate) use self::thread_summary::thread_settings_from_config_snapshot;

pub(crate) fn build_legacy_api_turns_from_rollout_items(items: &[RolloutItem]) -> Vec<Turn> {
    let mut builder = ThreadHistoryBuilder::new();
    for item in items {
        if is_persisted_rollout_item(item, kodex_protocol::protocol::ThreadHistoryMode::Legacy) {
            builder.handle_rollout_item(item);
        }
    }
    builder.finish()
}
