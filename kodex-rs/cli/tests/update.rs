use anyhow::Result;
use predicates::str::contains;
use std::path::Path;
use tempfile::TempDir;

fn kodex_command(kodex_home: &Path) -> Result<assert_cmd::Command> {
    let mut cmd = assert_cmd::Command::new(kodex_utils_cargo_bin::cargo_bin("kodex")?);
    cmd.env("KODEX_HOME", kodex_home);
    Ok(cmd)
}

#[cfg(debug_assertions)]
#[tokio::test]
async fn update_does_not_start_interactive_prompt() -> Result<()> {
    let kodex_home = TempDir::new()?;

    kodex_command(kodex_home.path())?
        .arg("update")
        .assert()
        .failure()
        .stderr(contains("`kodex update` is not available in debug builds"));

    Ok(())
}
