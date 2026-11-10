//! Spec validation, done before anything is created or granted.
//!
//! Every folder the sandbox is granted must sit strictly inside a KataBox-owned root
//! (marked with [`ROOT_MARKER`]). Drive roots, network paths, and the user profile or any
//! folder containing it are refused outright, so a bad spec can never hand the sandbox
//! the user's files.
#![cfg_attr(not(any(windows, target_os = "linux")), allow(dead_code))]

use std::path::{Path, PathBuf};

use crate::{SandboxError, SandboxSpec};

/// File that marks a folder as a KataBox-owned sandbox root. The app creates it in its
/// data folder; the dev/test root is created by `scripts/fetch-toolchains.{ps1,sh}`.
pub const ROOT_MARKER: &str = ".katabox-sandbox-root";

/// Canonical, validated paths from a spec. Backends use these, never the raw spec paths.
#[derive(Debug)]
pub(crate) struct Resolved {
    pub program: PathBuf,
    pub work_dir: PathBuf,
    pub read_only_dirs: Vec<PathBuf>,
}

fn bad<T>(msg: String) -> Result<T, SandboxError> {
    Err(SandboxError::InvalidSpec(msg))
}

pub(crate) fn resolve(spec: &SandboxSpec) -> Result<Resolved, SandboxError> {
    if spec.max_processes == 0 || spec.memory_bytes == 0 || spec.cpu_time.is_zero() {
        return bad("limits must be non-zero".into());
    }
    let profile = user_profile();
    let profile = profile.as_deref();

    let root = canonical(&spec.sandbox_root, "sandbox_root")?;
    refuse_sensitive(&root, profile, "sandbox_root")?;
    if !root.is_dir() {
        return bad(format!("sandbox_root is not a folder: {}", root.display()));
    }
    if !root.join(ROOT_MARKER).is_file() {
        return bad(format!(
            "sandbox_root {} is not a KataBox root (missing {ROOT_MARKER})",
            root.display()
        ));
    }

    let work_dir = canonical_dir_in_root(&spec.work_dir, &root, profile, "work_dir")?;
    let mut read_only_dirs = Vec::with_capacity(spec.read_only_dirs.len());
    for d in &spec.read_only_dirs {
        let d = canonical_dir_in_root(d, &root, profile, "read_only_dir")?;
        // A work folder inside a read-only folder would make part of it writable, and a
        // read-only folder inside the work folder would be deleted after the run.
        if work_dir.starts_with(&d) || d.starts_with(&work_dir) {
            return bad(format!(
                "work_dir {} overlaps read_only_dir {}",
                work_dir.display(),
                d.display()
            ));
        }
        read_only_dirs.push(d);
    }

    let program = canonical(&spec.program, "program")?;
    if !program.is_file() {
        return bad(format!("program is not a file: {}", program.display()));
    }
    if !read_only_dirs.iter().any(|d| program.starts_with(d)) {
        return bad(format!(
            "program {} is not inside any read_only_dir",
            program.display()
        ));
    }

    // Links in the work folder could redirect our ACL changes or deletion elsewhere.
    if let Some(link) = find_link(&work_dir)? {
        return bad(format!(
            "work_dir contains a link or reparse point: {}",
            link.display()
        ));
    }

    Ok(Resolved {
        program,
        work_dir,
        read_only_dirs,
    })
}

fn canonical_dir_in_root(
    p: &Path,
    root: &Path,
    profile: Option<&Path>,
    what: &str,
) -> Result<PathBuf, SandboxError> {
    let c = canonical(p, what)?;
    refuse_sensitive(&c, profile, what)?;
    if !c.is_dir() {
        return bad(format!("{what} is not a folder: {}", c.display()));
    }
    if c == root || !c.starts_with(root) {
        return bad(format!(
            "{what} {} must be strictly inside sandbox_root {}",
            c.display(),
            root.display()
        ));
    }
    Ok(c)
}

/// Absolute path with links resolved, in plain `C:\...` form.
fn canonical(p: &Path, what: &str) -> Result<PathBuf, SandboxError> {
    if !p.is_absolute() {
        return bad(format!("{what} must be absolute: {}", p.display()));
    }
    let c = std::fs::canonicalize(p).or_else(|e| bad(format!("{what} {}: {e}", p.display())))?;
    plain(c, what)
}

/// Strip the `\\?\` prefix Windows canonicalization adds; refuse network paths.
#[cfg(windows)]
fn plain(p: PathBuf, what: &str) -> Result<PathBuf, SandboxError> {
    let Some(s) = p.to_str() else {
        return bad(format!("{what} is not valid Unicode: {}", p.display()));
    };
    let rest = s.strip_prefix(r"\\?\").unwrap_or(s);
    let b = rest.as_bytes();
    if b.len() >= 3 && b[0].is_ascii_alphabetic() && b[1] == b':' && b[2] == b'\\' {
        Ok(PathBuf::from(rest))
    } else {
        bad(format!("{what} must be on a local drive: {}", p.display()))
    }
}

#[cfg(not(windows))]
fn plain(p: PathBuf, _what: &str) -> Result<PathBuf, SandboxError> {
    Ok(p)
}

/// Why `p` must never be granted to the sandbox, if it must not.
pub(crate) fn sensitive_reason(p: &Path, profile: Option<&Path>) -> Option<&'static str> {
    if p.parent().is_none() {
        Some("a drive root")
    } else if cfg!(target_os = "linux")
        && ["/proc", "/sys", "/dev", "/run"]
            .iter()
            .any(|sys| p.starts_with(sys))
    {
        Some("a system folder (/proc, /sys, /dev or /run)")
    } else if profile.is_some_and(|u| u.starts_with(p)) {
        Some("the user profile or a folder containing it")
    } else {
        None
    }
}

fn refuse_sensitive(p: &Path, profile: Option<&Path>, what: &str) -> Result<(), SandboxError> {
    match sensitive_reason(p, profile) {
        Some(why) => bad(format!("{what} {} is {why}", p.display())),
        None => Ok(()),
    }
}

fn user_profile() -> Option<PathBuf> {
    let var = if cfg!(windows) { "USERPROFILE" } else { "HOME" };
    let p = std::fs::canonicalize(std::env::var_os(var)?).ok()?;
    plain(p, var).ok()
}

/// True for symlinks, junctions and every other reparse point.
pub(crate) fn is_link(meta: &std::fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x400;
        meta.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0
    }
    #[cfg(not(windows))]
    {
        meta.file_type().is_symlink()
    }
}

/// First link found under `dir` (never descending into links).
fn find_link(dir: &Path) -> Result<Option<PathBuf>, SandboxError> {
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        let meta = std::fs::symlink_metadata(&path)?;
        if is_link(&meta) {
            return Ok(Some(path));
        }
        if meta.is_dir() {
            if let Some(found) = find_link(&path)? {
                return Ok(Some(found));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kb_core::{Language, Limits};

    /// root/{marker, tools/prog, work}, unique per test.
    struct Tree {
        root: PathBuf,
    }
    impl Tree {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!("kbv-{name}-{}", std::process::id()));
            let _ = std::fs::remove_dir_all(&root);
            std::fs::create_dir_all(root.join("tools")).unwrap();
            std::fs::create_dir_all(root.join("work")).unwrap();
            std::fs::write(root.join(ROOT_MARKER), "").unwrap();
            std::fs::write(root.join("tools/prog"), "").unwrap();
            Tree { root }
        }
        fn spec(&self) -> SandboxSpec {
            let mut s = SandboxSpec::new(
                &self.root,
                self.root.join("tools/prog"),
                self.root.join("work"),
                &Limits::default_for(Language::Python),
            );
            s.read_only_dirs = vec![self.root.join("tools")];
            s
        }
    }
    impl Drop for Tree {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn rejected(spec: &SandboxSpec, needle: &str) {
        match resolve(spec) {
            Err(SandboxError::InvalidSpec(m)) => assert!(m.contains(needle), "{m}"),
            other => panic!("expected rejection containing {needle:?}, got {other:?}"),
        }
    }

    #[test]
    fn valid_spec_resolves() {
        let t = Tree::new("ok");
        let r = resolve(&t.spec()).unwrap();
        assert!(r.work_dir.ends_with("work"));
    }

    #[test]
    fn root_without_marker_is_rejected() {
        let t = Tree::new("nomarker");
        std::fs::remove_file(t.root.join(ROOT_MARKER)).unwrap();
        rejected(&t.spec(), "not a KataBox root");
    }

    #[test]
    fn work_dir_outside_or_equal_to_root_is_rejected() {
        let t = Tree::new("outside");
        let other = Tree::new("outside-other");
        let mut s = t.spec();
        s.work_dir = other.root.join("work");
        rejected(&s, "strictly inside");
        s.work_dir = t.root.clone();
        rejected(&s, "strictly inside");
    }

    #[test]
    fn overlapping_dirs_are_rejected() {
        let t = Tree::new("overlap");
        std::fs::create_dir_all(t.root.join("tools/sub")).unwrap();
        let mut s = t.spec();
        s.work_dir = t.root.join("tools/sub");
        rejected(&s, "overlaps");
    }

    #[test]
    fn program_outside_read_only_dirs_is_rejected() {
        let t = Tree::new("prog");
        std::fs::write(t.root.join("work/evil"), "").unwrap();
        let mut s = t.spec();
        s.program = t.root.join("work/evil");
        rejected(&s, "not inside any read_only_dir");
    }

    #[test]
    fn relative_paths_are_rejected() {
        let t = Tree::new("relative");
        let mut s = t.spec();
        s.work_dir = PathBuf::from("work");
        rejected(&s, "must be absolute");
    }

    #[test]
    fn sensitive_paths_are_classified() {
        let profile = if cfg!(windows) {
            Path::new(r"C:\Users\someone")
        } else {
            Path::new("/home/someone")
        };
        let root = if cfg!(windows) { r"C:\" } else { "/" };
        assert_eq!(
            sensitive_reason(Path::new(root), Some(profile)),
            Some("a drive root")
        );
        assert!(sensitive_reason(profile, Some(profile)).is_some());
        assert!(sensitive_reason(profile.parent().unwrap(), Some(profile)).is_some());
        assert_eq!(
            sensitive_reason(&profile.join("AppData"), Some(profile)),
            None
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_system_folders_are_refused() {
        for p in ["/proc/self", "/sys/fs", "/dev/shm", "/run/user/1000"] {
            assert!(sensitive_reason(Path::new(p), None).is_some(), "{p}");
        }
        assert_eq!(sensitive_reason(Path::new("/opt/katabox"), None), None);
    }
}
