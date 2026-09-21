use std::path::PathBuf;

use kodex_utils_absolute_path::AbsolutePathBuf;

/// Paths and sandbox settings initialized when creating an executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecServerRuntimePaths {
    /// Stable path to the Kodex executable used to launch hidden helper modes.
    pub kodex_self_exe: AbsolutePathBuf,
    /// Path to the Linux sandbox helper alias used when the platform sandbox
    /// needs to re-enter Kodex by argv0.
    pub kodex_linux_sandbox_exe: Option<AbsolutePathBuf>,
    /// User-config opt-out of writable-root symlink checks beneath this host's home.
    #[cfg(target_os = "macos")]
    pub allowed_symlinked_kodex_home: Option<AbsolutePathBuf>,
}

impl ExecServerRuntimePaths {
    pub fn from_optional_paths(
        kodex_self_exe: Option<PathBuf>,
        kodex_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        let kodex_self_exe = kodex_self_exe.ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Kodex executable path is not configured",
            )
        })?;
        Self::new(kodex_self_exe, kodex_linux_sandbox_exe)
    }

    pub fn new(
        kodex_self_exe: PathBuf,
        kodex_linux_sandbox_exe: Option<PathBuf>,
    ) -> std::io::Result<Self> {
        Ok(Self {
            kodex_self_exe: absolute_path(kodex_self_exe)?,
            kodex_linux_sandbox_exe: kodex_linux_sandbox_exe.map(absolute_path).transpose()?,
            #[cfg(target_os = "macos")]
            allowed_symlinked_kodex_home: None,
        })
    }

    /// Applies the symlink opt-in resolved by the execution host's config loader.
    #[cfg(target_os = "macos")]
    pub fn with_allowed_symlinked_kodex_home(
        mut self,
        allowed_symlinked_kodex_home: Option<AbsolutePathBuf>,
    ) -> Self {
        self.allowed_symlinked_kodex_home = allowed_symlinked_kodex_home;
        self
    }
}

fn absolute_path(path: PathBuf) -> std::io::Result<AbsolutePathBuf> {
    AbsolutePathBuf::from_absolute_path(path.as_path())
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidInput, err))
}
