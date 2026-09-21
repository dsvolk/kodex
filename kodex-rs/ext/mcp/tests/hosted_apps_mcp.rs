use std::sync::Arc;

use kodex_config::McpServerTransportConfig;
use kodex_core::McpManager;
use kodex_core::config::Config;
use kodex_core::config::ConfigBuilder;
use kodex_core::plugins_manager_for_config;
use kodex_extension_api::ExtensionRegistryBuilder;
use kodex_extension_api::McpServerContribution;
use kodex_extension_api::McpServerContributionContext;
use kodex_extension_api::McpServerContributor;
use kodex_login::AuthManager;
use kodex_login::KodexAuth;
use kodex_login::test_support::auth_manager_from_optional_auth;
use kodex_mcp::KODEX_APPS_MCP_SERVER_NAME;
use pretty_assertions::assert_eq;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[tokio::test]
async fn contributes_hosted_plugin_runtime_without_an_executor() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            ("chatgpt_base_url".to_string(), "https://chatgpt.com".into()),
        ])
        .build()
        .await?;
    let auth = KodexAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(KODEX_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should be contributed as a configured server")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("hosted plugin runtime should use streamable HTTP");
    };
    assert_eq!(url, "https://chatgpt.com/backend-api/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn runtime_overlay_preserves_disabled_server() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.kodex_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
            ("mcp_servers.kodex_apps.enabled".to_string(), false.into()),
        ])
        .build()
        .await?;
    let auth = KodexAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(KODEX_APPS_MCP_SERVER_NAME)
        .ok_or("hosted plugin runtime should remain configured")?;

    assert!(!server.enabled());
    Ok(())
}

#[tokio::test]
async fn default_fallback_overwrites_reserved_config_without_an_extension() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), true.into()),
            (
                "mcp_servers.kodex_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let auth = KodexAuth::create_dummy_chatgpt_auth_for_testing();
    let manager = McpManager::new(Arc::new(plugins_manager_for_config(
        &config,
        AuthManager::from_auth_for_testing(auth.clone()),
    )));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    let server = servers
        .get(KODEX_APPS_MCP_SERVER_NAME)
        .ok_or("default Apps MCP should be present")?
        .config();
    let McpServerTransportConfig::StreamableHttp { url, .. } = &server.transport else {
        panic!("default Apps MCP should use streamable HTTP");
    };
    assert_eq!(url, "https://chatgpt.com/backend-api/ps/mcp");

    Ok(())
}

#[tokio::test]
async fn later_extension_can_remove_same_name_registration() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = KodexAuth::create_dummy_chatgpt_auth_for_testing();
    let mut builder = ExtensionRegistryBuilder::new();
    kodex_mcp_extension::install(&mut builder);
    builder.mcp_server_contributor(Arc::new(RemoveKodexApps));
    let manager = McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(
            &config,
            AuthManager::from_auth_for_testing(auth.clone()),
        )),
        Arc::new(builder.build()),
        kodex_core::KodexAppsToolsCache::default(),
    );

    let servers = manager.effective_servers(&config, Some(&auth)).await;

    assert!(!servers.contains_key(KODEX_APPS_MCP_SERVER_NAME));
    Ok(())
}

#[tokio::test]
async fn hosted_apps_mcp_requires_chatgpt_auth() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![("features.apps".to_string(), true.into())])
        .build()
        .await?;
    let auth = KodexAuth::from_api_key("test");
    let manager = installed_manager(&config, Some(auth.clone()));

    let servers = manager.effective_servers(&config, Some(&auth)).await;
    assert!(!servers.contains_key(KODEX_APPS_MCP_SERVER_NAME));

    Ok(())
}

#[tokio::test]
async fn disabled_apps_remove_reserved_server_config_for_all_hosts() -> TestResult {
    let kodex_home = tempfile::tempdir()?;
    let config = ConfigBuilder::default()
        .kodex_home(kodex_home.path().to_path_buf())
        .fallback_cwd(Some(kodex_home.path().to_path_buf()))
        .cli_overrides(vec![
            ("features.apps".to_string(), false.into()),
            (
                "mcp_servers.kodex_apps.url".to_string(),
                "https://example.com/mcp".into(),
            ),
        ])
        .build()
        .await?;
    let managers = [
        installed_manager(&config, /*auth*/ None),
        McpManager::new(Arc::new(plugins_manager_for_config(
            &config,
            auth_manager_from_optional_auth(/*auth*/ None),
        ))),
    ];
    for manager in managers {
        let servers = manager.runtime_servers(&config).await;
        assert!(!servers.contains_key(KODEX_APPS_MCP_SERVER_NAME));
    }
    Ok(())
}

fn installed_manager(config: &Config, auth: Option<KodexAuth>) -> McpManager {
    let mut builder = ExtensionRegistryBuilder::new();
    kodex_mcp_extension::install(&mut builder);
    McpManager::new_with_extensions(
        Arc::new(plugins_manager_for_config(
            config,
            auth_manager_from_optional_auth(auth),
        )),
        Arc::new(builder.build()),
        kodex_core::KodexAppsToolsCache::default(),
    )
}

struct RemoveKodexApps;

impl McpServerContributor<Config> for RemoveKodexApps {
    fn id(&self) -> &'static str {
        "remove_kodex_apps"
    }

    fn contribute<'a>(
        &'a self,
        _context: McpServerContributionContext<'a, Config>,
    ) -> kodex_extension_api::ExtensionFuture<'a, Vec<McpServerContribution>> {
        Box::pin(async move {
            vec![McpServerContribution::Remove {
                name: KODEX_APPS_MCP_SERVER_NAME.to_string(),
            }]
        })
    }
}
