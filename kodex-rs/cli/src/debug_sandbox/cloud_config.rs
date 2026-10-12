use kodex_cloud_config::cloud_config_bundle_loader_for_storage;
use kodex_config::CloudConfigBundleLoader;
use kodex_config::ConfigLoadOptions;
use kodex_core::config::bootstrap_auth_config;
use kodex_core::config::load_config_toml_with_layer_stack;
use kodex_utils_absolute_path::AbsolutePathBuf;
use toml::Value as TomlValue;

use super::DebugSandboxConfigOptions;
use super::ManagedRequirementsMode;

pub(super) async fn bootstrap_cloud_config_bundle(
    cli_overrides: &[(String, TomlValue)],
    options: &DebugSandboxConfigOptions,
    resolve_kodex_home: impl FnOnce() -> std::io::Result<AbsolutePathBuf>,
    strict_config: bool,
) -> anyhow::Result<CloudConfigBundleLoader> {
    if options.permissions_profile.is_none()
        || !matches!(
            options.managed_requirements_mode,
            ManagedRequirementsMode::Include
        )
    {
        return Ok(CloudConfigBundleLoader::default());
    }

    let kodex_home = resolve_kodex_home()?;
    let cwd = match options.cwd.as_deref() {
        Some(cwd) => AbsolutePathBuf::relative_to_current_dir(cwd)?,
        None => AbsolutePathBuf::current_dir()?,
    };
    let bootstrap_config = load_config_toml_with_layer_stack(
        kodex_home.as_path(),
        Some(&cwd),
        cli_overrides.to_vec(),
        ConfigLoadOptions {
            loader_overrides: options.loader_overrides.clone(),
            strict_config,
            cloud_config_bundle: CloudConfigBundleLoader::default(),
        },
    )
    .await?;
    Ok(cloud_config_bundle_loader_for_storage(
        bootstrap_auth_config(kodex_home.as_path(), &bootstrap_config)?,
        /*enable_kodex_api_key_env*/ false,
    )
    .await?)
}

#[cfg(test)]
#[path = "cloud_config_tests.rs"]
mod tests;
