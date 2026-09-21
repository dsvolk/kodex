use dirs::home_dir;
use kodex_utils_absolute_path::AbsolutePathBuf;
use std::path::PathBuf;

/// Returns the path to the Kodex configuration directory, which can be
/// specified by the `KODEX_HOME` environment variable. If not set, defaults to
/// `~/.kodex`.
///
/// - If `KODEX_HOME` is set, the value must exist and be a directory. The
///   value will be canonicalized and this function will Err otherwise.
/// - If `KODEX_HOME` is not set, this function does not verify that the
///   directory exists.
pub fn find_kodex_home() -> std::io::Result<AbsolutePathBuf> {
    let kodex_home_env = std::env::var("KODEX_HOME")
        .ok()
        .filter(|val| !val.is_empty());
    find_kodex_home_from_env(kodex_home_env.as_deref())
}

fn find_kodex_home_from_env(kodex_home_env: Option<&str>) -> std::io::Result<AbsolutePathBuf> {
    // Honor the `KODEX_HOME` environment variable when it is set to allow users
    // (and tests) to override the default location.
    match kodex_home_env {
        Some(val) => {
            let path = PathBuf::from(val);
            let metadata = std::fs::metadata(&path).map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("KODEX_HOME points to {val:?}, but that path does not exist"),
                ),
                _ => std::io::Error::new(
                    err.kind(),
                    format!("failed to read KODEX_HOME {val:?}: {err}"),
                ),
            })?;

            if !metadata.is_dir() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("KODEX_HOME points to {val:?}, but that path is not a directory"),
                ))
            } else {
                let canonical = path.canonicalize().map_err(|err| {
                    std::io::Error::new(
                        err.kind(),
                        format!("failed to canonicalize KODEX_HOME {val:?}: {err}"),
                    )
                })?;
                AbsolutePathBuf::from_absolute_path(canonical)
            }
        }
        None => {
            let mut p = home_dir().ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Could not find home directory",
                )
            })?;
            p.push(".kodex");
            AbsolutePathBuf::from_absolute_path(p)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::find_kodex_home_from_env;
    use dirs::home_dir;
    use kodex_utils_absolute_path::AbsolutePathBuf;
    use pretty_assertions::assert_eq;
    use std::fs;
    use std::io::ErrorKind;
    use tempfile::TempDir;

    #[test]
    fn find_kodex_home_env_missing_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let missing = temp_home.path().join("missing-kodex-home");
        let missing_str = missing
            .to_str()
            .expect("missing kodex home path should be valid utf-8");

        let err = find_kodex_home_from_env(Some(missing_str)).expect_err("missing KODEX_HOME");
        assert_eq!(err.kind(), ErrorKind::NotFound);
        assert!(
            err.to_string().contains("KODEX_HOME"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_kodex_home_env_file_path_is_fatal() {
        let temp_home = TempDir::new().expect("temp home");
        let file_path = temp_home.path().join("kodex-home.txt");
        fs::write(&file_path, "not a directory").expect("write temp file");
        let file_str = file_path
            .to_str()
            .expect("file kodex home path should be valid utf-8");

        let err = find_kodex_home_from_env(Some(file_str)).expect_err("file KODEX_HOME");
        assert_eq!(err.kind(), ErrorKind::InvalidInput);
        assert!(
            err.to_string().contains("not a directory"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn find_kodex_home_env_valid_directory_canonicalizes() {
        let temp_home = TempDir::new().expect("temp home");
        let temp_str = temp_home
            .path()
            .to_str()
            .expect("temp kodex home path should be valid utf-8");

        let resolved = find_kodex_home_from_env(Some(temp_str)).expect("valid KODEX_HOME");
        let expected = temp_home
            .path()
            .canonicalize()
            .expect("canonicalize temp home");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }

    #[test]
    fn find_kodex_home_without_env_uses_default_home_dir() {
        let resolved =
            find_kodex_home_from_env(/*kodex_home_env*/ None).expect("default KODEX_HOME");
        let mut expected = home_dir().expect("home dir");
        expected.push(".kodex");
        let expected = AbsolutePathBuf::from_absolute_path(expected).expect("absolute home");
        assert_eq!(resolved, expected);
    }
}
