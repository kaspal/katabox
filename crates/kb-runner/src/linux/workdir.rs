//! After-run cleanup of the work folder (ADR 0006 rules, Linux edition).
//!
//! Called only after the supervisor reported every sandboxed process dead. Nothing is
//! granted on disk on Linux (access comes from mounts and Landlock, both gone with the
//! processes), so cleanup removes what could mislead the host later: links, FIFOs,
//! sockets, device nodes and multiply-linked files are removed without being followed;
//! setuid/setgid/sticky bits are cleared; folders are made owner-writable so deletion
//! cannot be blocked by `chmod 0500`.

use std::fs;
use std::io;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
use std::path::Path;

use crate::SandboxError;

pub(crate) fn finish(work_dir: &Path, keep: bool) -> Result<(), SandboxError> {
    sanitize(work_dir)?;
    if keep {
        return Ok(());
    }
    match fs::remove_dir_all(work_dir) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(SandboxError::Io(io::Error::new(
            e.kind(),
            format!("could not delete work_dir {}: {e}", work_dir.display()),
        ))),
    }
}

fn ctx(op: &str, p: &Path) -> impl FnOnce(io::Error) -> SandboxError {
    let msg = format!("{op} {}", p.display());
    move |e| SandboxError::Io(io::Error::new(e.kind(), format!("{msg}: {e}")))
}

const SPECIAL_BITS: u32 = 0o7000; // setuid, setgid, sticky

/// Make `dir` itself usable, then walk its contents without following links.
fn sanitize(dir: &Path) -> Result<(), SandboxError> {
    let meta = fs::symlink_metadata(dir).map_err(ctx("symlink_metadata", dir))?;
    let mode = meta.mode() & 0o7777;
    let fixed = (mode & !SPECIAL_BITS) | 0o700;
    if fixed != mode {
        fs::set_permissions(dir, fs::Permissions::from_mode(fixed)).map_err(ctx("chmod", dir))?;
    }
    for entry in fs::read_dir(dir).map_err(ctx("read_dir", dir))? {
        let path = entry.map_err(ctx("read_dir entry in", dir))?.path();
        let meta = fs::symlink_metadata(&path).map_err(ctx("symlink_metadata", &path))?;
        let ft = meta.file_type();
        if ft.is_dir() {
            sanitize(&path)?;
        } else if ft.is_file() && meta.nlink() == 1 {
            let mode = meta.mode() & 0o7777;
            let fixed = (mode & !SPECIAL_BITS) | 0o600;
            if fixed != mode {
                fs::set_permissions(&path, fs::Permissions::from_mode(fixed))
                    .map_err(ctx("chmod", &path))?;
            }
        } else {
            // Symlink, FIFO, socket, device node, or a hard link that shares its inode
            // with a file elsewhere (changing its mode would change that file too).
            fs::remove_file(&path).map_err(ctx("remove", &path))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::symlink;

    #[test]
    fn sanitize_removes_links_and_specials_without_following() {
        let base = std::env::temp_dir().join(format!("kbw-{}", std::process::id()));
        let _ = fs::remove_dir_all(&base);
        let work = base.join("work");
        let outside = base.join("outside");
        fs::create_dir_all(work.join("sub")).unwrap();
        fs::create_dir_all(&outside).unwrap();
        fs::write(outside.join("keep.txt"), "x").unwrap();
        fs::set_permissions(outside.join("keep.txt"), fs::Permissions::from_mode(0o4755)).unwrap();
        fs::write(work.join("own.txt"), "x").unwrap();
        fs::set_permissions(work.join("own.txt"), fs::Permissions::from_mode(0o6755)).unwrap();
        symlink(&outside, work.join("link-dir")).unwrap();
        fs::hard_link(outside.join("keep.txt"), work.join("sub/hard.txt")).unwrap();
        nix::unistd::mkfifo(&work.join("fifo"), nix::sys::stat::Mode::S_IRWXU).unwrap();
        fs::set_permissions(work.join("sub"), fs::Permissions::from_mode(0o500)).unwrap();

        finish(&work, true).unwrap();
        assert!(fs::symlink_metadata(work.join("link-dir")).is_err());
        assert!(fs::symlink_metadata(work.join("fifo")).is_err());
        assert!(!work.join("sub/hard.txt").exists());
        assert_eq!(
            fs::metadata(work.join("own.txt")).unwrap().mode() & 0o7777,
            0o755
        );
        // The link targets are untouched.
        assert_eq!(
            fs::metadata(outside.join("keep.txt")).unwrap().mode() & 0o7777,
            0o4755
        );

        finish(&work, false).unwrap();
        assert!(!work.exists());
        assert!(outside.join("keep.txt").is_file());
        fs::remove_dir_all(&base).unwrap();
    }
}
