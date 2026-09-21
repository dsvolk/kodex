#[cfg(target_os = "windows")]
fn main() -> anyhow::Result<()> {
    kodex_windows_sandbox::setup_helper_main()
}

#[cfg(not(target_os = "windows"))]
fn main() {
    panic!("kodex-windows-sandbox-setup is Windows-only");
}
