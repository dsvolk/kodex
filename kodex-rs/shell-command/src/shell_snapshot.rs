//! Capture, prepare credential-safe state, and render restorable shell snapshots.

#[path = "shell_snapshot_capture.rs"]
mod capture;
#[path = "shell_snapshot_credentials.rs"]
mod credentials;
#[path = "shell_snapshot_exports.rs"]
mod exports;
#[path = "shell_snapshot_literals.rs"]
mod literals;
#[path = "shell_snapshot_render.rs"]
mod render;

pub use capture::CapturedSnapshot;
pub use capture::CapturedStartupEnvironment;
pub use capture::SnapshotCaptureOptions;
pub use capture::snapshot_capture_script;
pub use credentials::PreparedSnapshot;
pub use credentials::SnapshotCredentialEnvironment;
pub use credentials::prepare_snapshot_credentials;

use capture::BASH_SH_SNAPSHOT_HEADER;

#[cfg(all(test, unix))]
#[path = "shell_snapshot_tests.rs"]
mod tests;

/// Returns the POSIX shell helper used to resolve supported `ENV` startup paths.
///
/// The helper deliberately supports only non-evaluating path forms. Unsupported
/// shell expressions are returned unchanged rather than executed.
pub fn posix_env_path_expansion_function() -> &'static str {
    r#"__kodex_snapshot_expand_env() (
  set +u
  __kodex_snapshot_getenv() {
    # Preserve the value separately from lookup status and its formatting newline.
    __kodex_env_expanded=$(
      if command -v printenv >/dev/null 2>&1; then
        printenv "$1"
      elif [ "$1" = PATH ]; then
        [ "${PATH+x}" = x ] && printf '%s\n' "$PATH"
      else
        command -p printenv "$1"
      fi && printf '.'
    ) || return 1
    __kodex_env_expanded=${__kodex_env_expanded%?}
    __kodex_env_expanded=${__kodex_env_expanded%?}
  }
  __kodex_env_file=$1
  case "$__kodex_env_file" in
    '~/'*) __kodex_env_file="${HOME-}/${__kodex_env_file#*/}" ;;
    '${PATH%%:*}') __kodex_env_file="${PATH%%:*}" ;;
    '${PATH%%:*}/'*) __kodex_env_file="${PATH%%:*}/${__kodex_env_file#*/}" ;;
    '${'*)
      __kodex_env_body=${__kodex_env_file#\$\{}
      case "$__kodex_env_body" in
        *\}*)
          __kodex_env_name=${__kodex_env_body%%\}*}
          __kodex_env_suffix=${__kodex_env_body#*\}}
          case "$__kodex_env_name" in
            *:-*)
              __kodex_env_default=${__kodex_env_name#*:-}
              __kodex_env_name=${__kodex_env_name%%:-*}
              __kodex_env_has_default=1
              ;;
            *) __kodex_env_has_default= ;;
          esac
          case "$__kodex_env_name" in
            ''|[0-9]*|*[!A-Za-z0-9_]*) ;;
            *)
              case "$__kodex_env_suffix" in
                ''|/*)
                  if __kodex_snapshot_getenv "$__kodex_env_name" 2>/dev/null &&
                    { [ -n "$__kodex_env_expanded" ] || [ -z "$__kodex_env_has_default" ]; }; then
                    __kodex_env_file="$__kodex_env_expanded$__kodex_env_suffix"
                  elif [ -n "$__kodex_env_has_default" ]; then
                    __kodex_env_default=$(__kodex_snapshot_expand_env "$__kodex_env_default")
                    __kodex_env_file="$__kodex_env_default$__kodex_env_suffix"
                  fi
                  ;;
              esac
              ;;
          esac
          ;;
      esac
      ;;
    '$'*)
      __kodex_env_name=${__kodex_env_file%%/*}
      __kodex_env_name=${__kodex_env_name#\$}
      case "$__kodex_env_name" in
        ''|[0-9]*|*[!A-Za-z0-9_]*) ;;
        *)
          if __kodex_snapshot_getenv "$__kodex_env_name" 2>/dev/null; then
            if [ "$__kodex_env_file" = "\$$__kodex_env_name" ]; then
              __kodex_env_file=$__kodex_env_expanded
            else
              __kodex_env_file="$__kodex_env_expanded/${__kodex_env_file#*/}"
            fi
          fi
          ;;
      esac
      ;;
  esac
  printf '%s' "$__kodex_env_file"
)"#
}

#[derive(Clone, Copy)]
pub enum SnapshotStartup {
    Interactive,
    NonInteractive,
}
