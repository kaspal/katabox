//! Uninstall cleanup (ADR 0021): the installers run `katabox-desktop.exe --uninstall-cleanup`
//! before removing the app. It revokes the sandbox's persistent folder grants and
//! deletes the AppContainer profile through kb-runner's existing [`kb_runner::cleanup`].
//! It never deletes files: removing the user's data is the installer's choice.

use std::path::{Path, PathBuf};

use kb_store::UserDb;
use serde_json::Value;

use crate::root::SANDBOX_ROOT_SETTING;
use crate::{AppError, AppErrorCode, Result};

/// What the cleanup touched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CleanupReport {
    /// The sandbox roots found (the configured one and the default one, if marked).
    pub roots: Vec<PathBuf>,
    /// The folders whose grants were revoked.
    pub dirs: Vec<PathBuf>,
}

/// Folders under `root` that can carry a persistent sandbox grant: each toolchain
/// (read-only grants), each compiled-binary cache and any leftover run folder.
pub fn grant_dirs(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for sub in ["toolchains", "cache", "runs"] {
        let Ok(entries) = std::fs::read_dir(root.join(sub)) else {
            continue;
        };
        let mut dirs: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_ok_and(|t| t.is_dir()))
            .map(|e| e.path())
            .collect();
        dirs.sort();
        out.extend(dirs);
    }
    out
}

fn is_root(p: &Path) -> bool {
    p.join(kb_runner::ROOT_MARKER).is_file()
}

/// The roots this installation may have used: the `sandbox_root` setting (read without
/// migrating user.db) and `<local data>/sandbox`. The env var and the dev root are
/// development-only and not considered.
pub async fn sandbox_roots(local_data: &Path, user_db: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(Some(Value::String(s))) = UserDb::peek_setting(user_db, SANDBOX_ROOT_SETTING).await {
        let p = PathBuf::from(s);
        if is_root(&p) {
            roots.push(p);
        }
    }
    let default = local_data.join("sandbox");
    if is_root(&default) && !roots.contains(&default) {
        roots.push(default);
    }
    roots
}

/// Revoke every grant and delete the AppContainer profile. Safe to run twice; with
/// nothing installed it still removes a leftover profile.
pub async fn cleanup_sandbox(local_data: &Path, user_db: &Path) -> Result<CleanupReport> {
    let roots = sandbox_roots(local_data, user_db).await;
    let dirs: Vec<PathBuf> = roots.iter().flat_map(|r| grant_dirs(r)).collect();
    // Under the cross-process sandbox lock, so a running judge elsewhere finishes first.
    kb_judge::sandbox::with_locks(&roots, || kb_runner::cleanup(&dirs))
        .map_err(|e| AppError::new(AppErrorCode::Sandbox, format!("cleanup lock: {e}")))?
        .map_err(|e| AppError::new(AppErrorCode::Sandbox, format!("cleanup: {e}")))?;
    Ok(CleanupReport { roots, dirs })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testutil::temp_dir;

    #[tokio::test]
    async fn finds_the_roots_and_their_grant_folders() {
        let t = temp_dir("uninstall");
        let local = t.join("local");
        let root = local.join("sandbox");
        for d in [
            "toolchains/python-3.14",
            "toolchains/node-24",
            "cache/rust",
            "runs/r1",
            "tools/basedpyright",
        ] {
            std::fs::create_dir_all(root.join(d)).unwrap();
        }
        std::fs::write(root.join("toolchains/readme.txt"), "").unwrap();
        let user_db = t.join("roaming/user.db");
        // Not a marked root yet: nothing to clean.
        assert!(sandbox_roots(&local, &user_db).await.is_empty());
        std::fs::write(root.join(kb_runner::ROOT_MARKER), "").unwrap();

        // A configured root in user.db counts too.
        let custom = t.join("custom");
        std::fs::create_dir_all(custom.join("toolchains/php-8.5")).unwrap();
        std::fs::write(custom.join(kb_runner::ROOT_MARKER), "").unwrap();
        let db = UserDb::open(&user_db).await.unwrap();
        db.set_setting(
            SANDBOX_ROOT_SETTING,
            &Value::String(custom.display().to_string()),
        )
        .await
        .unwrap();
        db.close().await;

        assert_eq!(
            sandbox_roots(&local, &user_db).await,
            [custom.clone(), root.clone()]
        );
        let names: Vec<String> = grant_dirs(&root)
            .iter()
            .map(|p| {
                p.strip_prefix(&root)
                    .unwrap()
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect();
        assert_eq!(
            names,
            [
                "toolchains/node-24",
                "toolchains/python-3.14",
                "cache/rust",
                "runs/r1"
            ]
        );
        assert!(grant_dirs(&t.join("missing")).is_empty());
    }
}
