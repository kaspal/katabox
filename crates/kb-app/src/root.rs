//! Choosing the sandbox root. kb-runner only grants folders strictly inside a root
//! marked with [`kb_runner::ROOT_MARKER`], and the toolchains must be inside it too, so
//! the root and the toolchains (`<root>/toolchains/...`) are chosen together.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::{AppError, AppErrorCode, Result};

/// Environment variable naming the sandbox root (highest precedence).
pub const SANDBOX_ROOT_ENV: &str = "KATABOX_SANDBOX_ROOT";
/// user.db setting naming the sandbox root (a JSON string).
pub const SANDBOX_ROOT_SETTING: &str = "sandbox_root";

/// Where the sandbox root came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(rename_all = "snake_case")]
pub enum RootSource {
    /// `KATABOX_SANDBOX_ROOT`.
    Env,
    /// The `sandbox_root` setting.
    Setting,
    /// Development: the fetch script's `target/katabox`.
    Dev,
    /// `<app local data>/sandbox` (the default; created when missing).
    AppData,
}

/// First match wins: env var, setting, dev root (only if it is already a marked root),
/// then the app's local data folder, created and marked when missing. An env var or
/// setting must point at an existing marked root: the app never marks a folder the user
/// named.
pub(crate) fn resolve(
    env: Option<PathBuf>,
    setting: Option<PathBuf>,
    dev: Option<&Path>,
    local_data: &Path,
) -> Result<(PathBuf, RootSource)> {
    let configured = env
        .filter(|p| !p.as_os_str().is_empty())
        .map(|p| (p, RootSource::Env))
        .or_else(|| setting.map(|p| (p, RootSource::Setting)));
    if let Some((p, source)) = configured {
        if !is_root(&p) {
            return Err(AppError::new(
                AppErrorCode::Sandbox,
                format!(
                    "{} is not a KataBox sandbox root (missing {}); from {source:?}",
                    p.display(),
                    kb_runner::ROOT_MARKER
                ),
            ));
        }
        return Ok((p, source));
    }
    if let Some(d) = dev.filter(|d| is_root(d)) {
        return Ok((d.to_path_buf(), RootSource::Dev));
    }
    let root = local_data.join("sandbox");
    if !is_root(&root) {
        std::fs::create_dir_all(root.join("toolchains"))
            .and_then(|()| std::fs::write(root.join(kb_runner::ROOT_MARKER), ""))
            .map_err(|e| {
                AppError::new(
                    AppErrorCode::Sandbox,
                    format!("cannot create the sandbox root {}: {e}", root.display()),
                )
            })?;
    }
    Ok((root, RootSource::AppData))
}

fn is_root(p: &Path) -> bool {
    p.join(kb_runner::ROOT_MARKER).is_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;

    fn marked(dir: &Path) -> PathBuf {
        std::fs::create_dir_all(dir).unwrap();
        std::fs::write(dir.join(kb_runner::ROOT_MARKER), "").unwrap();
        dir.to_path_buf()
    }

    #[test]
    fn precedence_env_setting_dev_appdata() {
        let t = temp_dir("root");
        let (env, setting, dev) = (
            marked(&t.join("env")),
            marked(&t.join("setting")),
            marked(&t.join("dev")),
        );
        let local = t.join("local");
        let r = |e: Option<&Path>, s: Option<&Path>, d: Option<&Path>| {
            resolve(
                e.map(Path::to_path_buf),
                s.map(Path::to_path_buf),
                d,
                &local,
            )
            .unwrap()
        };
        assert_eq!(
            r(Some(&env), Some(&setting), Some(&dev)),
            (env.clone(), RootSource::Env)
        );
        assert_eq!(
            r(None, Some(&setting), Some(&dev)),
            (setting.clone(), RootSource::Setting)
        );
        assert_eq!(r(None, None, Some(&dev)), (dev.clone(), RootSource::Dev));
        // An unmarked dev root is skipped; the app data root is created and marked.
        let (root, src) = r(None, None, Some(&t.join("nothing-here")));
        assert_eq!(
            (root.clone(), src),
            (local.join("sandbox"), RootSource::AppData)
        );
        assert!(root.join(kb_runner::ROOT_MARKER).is_file());
        assert!(root.join("toolchains").is_dir());
        // An empty env var counts as unset.
        assert_eq!(r(Some(Path::new("")), None, None).1, RootSource::AppData);
    }

    #[test]
    fn a_named_root_must_already_be_marked() {
        let t = temp_dir("unmarked");
        let plain = t.join("plain");
        std::fs::create_dir_all(&plain).unwrap();
        let err = resolve(Some(plain.clone()), None, None, &t).unwrap_err();
        assert_eq!(err.code, AppErrorCode::Sandbox);
        assert!(!plain.join(kb_runner::ROOT_MARKER).exists());
        let err = resolve(None, Some(plain), None, &t).unwrap_err();
        assert!(err.message.contains("Setting"), "{}", err.message);
    }
}
