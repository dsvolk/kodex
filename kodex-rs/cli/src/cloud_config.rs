use anyhow::Context;
use anyhow::Result;
use kodex_cloud_config::cloud_config_bundle_loader_for_storage;
use kodex_config::CloudConfigBundleLoader;
use kodex_config::ConfigLoadOptions;
use kodex_core::config::Config;
use kodex_core::config::ConfigBuilder;
use kodex_core::config::ConfigOverrides;
use kodex_core::config::LoaderOverrides;
use kodex_core::config::bootstrap_auth_config;
use kodex_core::config::find_kodex_home;
use kodex_core::config::load_config_toml_with_layer_stack;
use kodex_utils_absolute_path::AbsolutePathBuf;
use kodex_utils_cli::CliConfigOverrides;

pub(crate) async fn load_config(
    config_overrides: &CliConfigOverrides,
    loader_overrides: LoaderOverrides,
) -> Result<Config> {
    config_builder(
        config_overrides,
        loader_overrides,
        ConfigOverrides::default(),
    )
    .await?
    .build()
    .await
    .context("failed to load configuration")
}

pub(crate) async fn config_builder(
    config_overrides: &CliConfigOverrides,
    loader_overrides: LoaderOverrides,
    harness_overrides: ConfigOverrides,
) -> Result<ConfigBuilder> {
    let cli_overrides = config_overrides
        .parse_overrides()
        .map_err(anyhow::Error::msg)?;
    let kodex_home = find_kodex_home().context("failed to resolve KODEX_HOME")?;
    let cwd = match harness_overrides.cwd.as_deref() {
        Some(cwd) => AbsolutePathBuf::relative_to_current_dir(cwd),
        None => AbsolutePathBuf::current_dir(),
    }
    .context("failed to resolve current directory")?;
    let bootstrap_config = load_config_toml_with_layer_stack(
        kodex_home.as_path(),
        Some(&cwd),
        cli_overrides.clone(),
        ConfigLoadOptions {
            loader_overrides: loader_overrides.clone(),
            strict_config: false,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await
    .context("failed to load bootstrap configuration")?;
    let cloud_config_bundle = cloud_config_bundle_loader_for_storage(
        bootstrap_auth_config(kodex_home.as_path(), &bootstrap_config)
            .context("failed to resolve cloud configuration authentication")?,
        /*enable_kodex_api_key_env*/ false,
    )
    .await
    .context("failed to initialize cloud configuration authentication")?;

    Ok(ConfigBuilder::default()
        .kodex_home(kodex_home.to_path_buf())
        .cli_overrides(cli_overrides)
        .loader_overrides(loader_overrides)
        .harness_overrides(harness_overrides)
        .cloud_config_bundle(cloud_config_bundle)
        .fallback_cwd(Some(cwd.to_path_buf())))
}
