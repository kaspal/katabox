//! Paths inside a sandbox root. The judge, the language-server bridge and the installer
//! all take their folders from here (and the manifest's component `dest`s).
//!
//! ```text
//! <root>/.katabox-sandbox-root     marker (kb-runner refuses unmarked roots)
//! <root>/toolchains/<runtime>      judge runtimes (granted read-only to the sandbox per run)
//! <root>/tools/...                 language servers (never granted; ADR 0014)
//! <root>/receipts/<component>.json written last: what is installed
//! <root>/staging/                  extraction and trash (purged)
//! <root>/downloads/                verified and partial downloads (.part, resumable)
//! <root>/.install.lock             cross-process install lock
//! ```

use std::path::{Component as PathComponent, Path, PathBuf};

use crate::manifest::{Component, Manifest, Platform};
use crate::{Error, Result};

/// The marker file every sandbox root carries (same as kb-runner's `ROOT_MARKER`).
pub const ROOT_MARKER: &str = ".katabox-sandbox-root";

pub const TOOLCHAINS: &str = "toolchains";
pub const TOOLS: &str = "tools";
pub const RECEIPTS: &str = "receipts";
pub const STAGING: &str = "staging";
pub const DOWNLOADS: &str = "downloads";
pub const LOCK_FILE: &str = ".install.lock";

/// Folders under the root, resolved from the manifest for this platform.
#[derive(Debug, Clone)]
pub struct Layout {
    pub root: PathBuf,
    pub platform: Platform,
    pub manifest: &'static Manifest,
}

impl Layout {
    /// The layout of the built-in manifest.
    pub fn new(root: impl Into<PathBuf>, platform: Platform) -> Layout {
        Layout::with_manifest(root, platform, Manifest::builtin())
    }

    pub fn with_manifest(
        root: impl Into<PathBuf>,
        platform: Platform,
        manifest: &'static Manifest,
    ) -> Layout {
        Layout {
            root: root.into(),
            platform,
            manifest,
        }
    }

    /// Where a component lives, e.g. `toolchains/python-3.14`.
    pub fn component_dir(&self, id: &str) -> Option<PathBuf> {
        self.manifest
            .component(id, self.platform)
            .map(|c| self.dest(c))
    }

    pub(crate) fn dest(&self, c: &Component) -> PathBuf {
        let mut p = self.root.clone();
        for part in c.dest.split('/') {
            p.push(part);
        }
        p
    }

    /// The Node runtime folder (language servers run on it).
    pub fn node_dir(&self) -> PathBuf {
        self.must("node")
    }
    pub fn python_dir(&self) -> PathBuf {
        self.must("python")
    }
    pub fn php_dir(&self) -> PathBuf {
        self.component_dir("php")
            .unwrap_or_else(|| self.root.join(TOOLCHAINS).join("php-8.5"))
    }
    pub fn rust_sysroot(&self) -> PathBuf {
        self.component_dir("rust")
            .unwrap_or_else(|| self.root.join(TOOLCHAINS).join("rust-1.99"))
    }
    /// std's sources for rust-analyzer (`library/` inside).
    pub fn rust_src(&self) -> PathBuf {
        self.component_dir("rust-src")
            .unwrap_or_else(|| self.root.join(TOOLS).join("rust-src"))
    }
    pub fn rust_analyzer_dir(&self) -> PathBuf {
        self.component_dir("rust-analyzer")
            .unwrap_or_else(|| self.root.join(TOOLS).join("rust-analyzer"))
    }
    /// `tools/language-servers/node_modules`.
    pub fn node_modules(&self) -> PathBuf {
        self.root
            .join(TOOLS)
            .join("language-servers")
            .join("node_modules")
    }
    pub fn tools(&self) -> PathBuf {
        self.root.join(TOOLS)
    }
    pub fn receipts(&self) -> PathBuf {
        self.root.join(RECEIPTS)
    }
    pub fn staging(&self) -> PathBuf {
        self.root.join(STAGING)
    }
    pub fn downloads(&self) -> PathBuf {
        self.root.join(DOWNLOADS)
    }
    /// The judge's compiled-binary cache for Rust (cleared when Rust changes).
    pub fn rust_cache(&self) -> PathBuf {
        self.root.join("cache").join("rust")
    }

    fn must(&self, id: &str) -> PathBuf {
        self.component_dir(id).unwrap_or_else(|| {
            let name = if id == "python" {
                "python-3.14"
            } else {
                "node-26"
            };
            self.root.join(TOOLCHAINS).join(name)
        })
    }
}

/// A manifest `dest`: relative, `/`-separated, plain names, under `toolchains/` or `tools/`.
pub(crate) fn check_dest(dest: &str) -> std::result::Result<(), String> {
    let parts: Vec<&str> = dest.split('/').collect();
    if parts.len() < 2 || !matches!(parts[0], TOOLCHAINS | TOOLS) {
        return Err(format!("dest {dest:?} must be under toolchains/ or tools/"));
    }
    for p in &parts {
        if p.is_empty()
            || *p == "."
            || *p == ".."
            || !p
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
        {
            return Err(format!("dest {dest:?} has an invalid part {p:?}"));
        }
    }
    Ok(())
}

/// Make `dir` (under `root`) a real folder: create what is missing, and refuse any part
/// that is a symlink, junction or other reparse point, so renames can never leave the root.
pub(crate) fn ensure_real_dir(root: &Path, dir: &Path) -> Result<()> {
    let rel = dir.strip_prefix(root).map_err(|_| Error::InvalidRoot {
        path: dir.to_path_buf(),
        why: "outside the sandbox root".into(),
    })?;
    let mut cur = root.to_path_buf();
    for part in rel.components() {
        let PathComponent::Normal(name) = part else {
            return Err(Error::InvalidRoot {
                path: dir.to_path_buf(),
                why: "not a plain path".into(),
            });
        };
        cur.push(name);
        match std::fs::symlink_metadata(&cur) {
            Ok(m) => {
                if !m.is_dir() || is_link(&m) {
                    return Err(Error::InvalidRoot {
                        path: cur.clone(),
                        why: "must be a plain folder (not a link or junction)".into(),
                    });
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&cur)
                    .or_else(|e| {
                        if e.kind() == std::io::ErrorKind::AlreadyExists {
                            Ok(())
                        } else {
                            Err(e)
                        }
                    })
                    .map_err(|e| Error::io(format!("create {}", cur.display()), e))?;
            }
            Err(e) => return Err(Error::io(format!("inspect {}", cur.display()), e)),
        }
    }
    Ok(())
}

/// A symlink, junction or other reparse point.
pub(crate) fn is_link(m: &std::fs::Metadata) -> bool {
    if m.file_type().is_symlink() {
        return true;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        if m.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return true;
        }
    }
    false
}

/// Check that `root` is a marked sandbox root (a plain folder carrying the marker).
pub fn check_root(root: &Path) -> Result<PathBuf> {
    let invalid = |why: &str| Error::InvalidRoot {
        path: root.to_path_buf(),
        why: why.into(),
    };
    let m = std::fs::symlink_metadata(root).map_err(|_| invalid("does not exist"))?;
    if !m.is_dir() || is_link(&m) {
        return Err(invalid("must be a plain folder"));
    }
    if !root.join(ROOT_MARKER).is_file() {
        return Err(invalid("missing the .katabox-sandbox-root marker"));
    }
    std::path::absolute(root).map_err(|e| Error::io("resolve the sandbox root", e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dests_are_plain_and_inside_known_folders() {
        assert!(check_dest("toolchains/python-3.14").is_ok());
        assert!(check_dest("tools/language-servers/node_modules/typescript").is_ok());
        for bad in [
            "python",
            "toolchains",
            "toolchains/../x",
            "toolchains//x",
            "cache/x",
            "/toolchains/x",
            "toolchains/a b",
            "toolchains/x\\y",
        ] {
            assert!(check_dest(bad).is_err(), "{bad}");
        }
    }

    #[test]
    fn layout_matches_what_the_judge_and_bridge_expect() {
        // The judge's dev layout (kb-judge JudgeEnv::dev) and kb-bridge use these names.
        let l = Layout::new("R", Platform::WindowsX86_64);
        let root = Path::new("R");
        assert_eq!(l.python_dir(), root.join("toolchains").join("python-3.14"));
        assert_eq!(l.node_dir(), root.join("toolchains").join("node-26"));
        assert_eq!(l.php_dir(), root.join("toolchains").join("php-8.5"));
        assert_eq!(l.rust_sysroot(), root.join("toolchains").join("rust-1.99"));
        assert_eq!(l.rust_src(), root.join("tools").join("rust-src"));
        assert_eq!(
            l.rust_analyzer_dir(),
            root.join("tools").join("rust-analyzer")
        );
        assert_eq!(
            l.component_dir("basedpyright").unwrap(),
            l.node_modules().join("basedpyright")
        );
        for p in Platform::ALL {
            let l = Layout::new("R", p);
            assert_eq!(l.python_dir(), root.join("toolchains").join("python-3.14"));
            assert_eq!(l.node_dir(), root.join("toolchains").join("node-26"));
        }
    }

    #[test]
    fn real_dirs_refuse_links() {
        let t = tempfile::tempdir().unwrap();
        let root = t.path();
        ensure_real_dir(root, &root.join("a").join("b")).unwrap();
        assert!(root.join("a").join("b").is_dir());
        std::fs::write(root.join("f"), "x").unwrap();
        assert!(ensure_real_dir(root, &root.join("f").join("x")).is_err());
        assert!(ensure_real_dir(root, Path::new("/elsewhere")).is_err());
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(root.join("a"), root.join("link")).unwrap();
            assert!(ensure_real_dir(root, &root.join("link").join("x")).is_err());
        }
    }
}
