use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

use crate::AppPaths;

pub fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

/// A fresh, empty temp folder for one test.
pub fn temp_dir(name: &str) -> PathBuf {
    static N: AtomicU32 = AtomicU32::new(0);
    let dir = std::env::temp_dir().join(format!(
        "kb-app-{name}-{}-{}",
        std::process::id(),
        N.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// content.db built once per test process from `content/problems`.
pub fn content_db() -> PathBuf {
    static DB: OnceLock<PathBuf> = OnceLock::new();
    DB.get_or_init(|| {
        let out = temp_dir("content").join("content.db");
        let o = out.clone();
        // kb-pack runs its own runtime; build outside the test's runtime.
        std::thread::spawn(move || {
            kb_pack::build_pack(&repo().join("content/problems"), &o, "test", "1.0.0").unwrap()
        })
        .join()
        .unwrap();
        out
    })
    .clone()
}

/// Paths for an app in a fresh temp folder, with no dev root.
pub fn paths(name: &str) -> AppPaths {
    let dir = temp_dir(name);
    AppPaths {
        content_db: content_db(),
        user_db: dir.join("data/user.db"),
        local_data_dir: dir.join("local"),
        dev_root: None,
        dev_data: false,
    }
}
