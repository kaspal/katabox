//! After-run cleanup of the work folder, so nothing stays writable between runs.
//!
//! Called only after the job is closed (every sandboxed process is dead), so the tree
//! cannot change underneath us.

use std::fs;
use std::io;
use std::os::windows::fs::MetadataExt;
use std::path::Path;
use std::thread;
use std::time::Duration;

use super::acl;
use super::appcontainer::AppContainerSid;
use crate::SandboxError;
use crate::validate::is_link;

const FILE_ATTRIBUTE_DIRECTORY: u32 = 0x10;
const DELETE_ATTEMPTS: u32 = 20;
const DELETE_BACKOFF: Duration = Duration::from_millis(100);

pub(crate) fn finish(
    work_dir: &Path,
    sid: &AppContainerSid,
    keep: bool,
) -> Result<(), SandboxError> {
    // 1. Links the sandbox made could point our ACL calls at folders outside; remove
    //    the links themselves (never their targets) before touching permissions.
    remove_links(work_dir)?;
    // 2. Drop the run's grant. Inherited copies on the contents vanish with it.
    acl::revoke(work_dir, sid)?;
    if keep {
        // 3a. Undo any explicit permission the sandbox put on its own files (e.g. a grant
        //     to ALL APPLICATION PACKAGES, which every later run would match).
        return reset_tree(work_dir);
    }
    // 3b. Delete. If that fails, still reset so leftovers are not accessible, then report.
    match remove_with_retry(work_dir) {
        Ok(()) => Ok(()),
        Err(e) => {
            reset_tree(work_dir)?;
            Err(SandboxError::Io(io::Error::new(
                e.kind(),
                format!("could not delete work_dir {}: {e}", work_dir.display()),
            )))
        }
    }
}

/// Attach the operation and path to an I/O error.
fn ctx(op: &str, p: &Path) -> impl FnOnce(io::Error) -> SandboxError {
    let msg = format!("{op} {}", p.display());
    move |e| SandboxError::Io(io::Error::new(e.kind(), format!("{msg}: {e}")))
}

/// Remove every link or reparse point under `dir` without following it.
fn remove_links(dir: &Path) -> Result<(), SandboxError> {
    for entry in fs::read_dir(dir).map_err(ctx("read_dir", dir))? {
        let path = entry.map_err(ctx("read_dir entry in", dir))?.path();
        let meta = fs::symlink_metadata(&path).map_err(ctx("symlink_metadata", &path))?;
        if is_link(&meta) {
            // A directory junction/symlink is removed with remove_dir, which deletes the
            // link and leaves the target alone. `meta.is_dir()` is false for links, so
            // use the directory attribute.
            if meta.file_attributes() & FILE_ATTRIBUTE_DIRECTORY != 0 {
                fs::remove_dir(&path).map_err(ctx("remove link (dir)", &path))?;
            } else {
                fs::remove_file(&path).map_err(ctx("remove link (file)", &path))?;
            }
        } else if meta.is_dir() {
            remove_links(&path)?;
        }
    }
    Ok(())
}

/// Reset every entry under `dir` (not `dir` itself) to inherited-only permissions.
/// Assumes `remove_links` already ran.
fn reset_tree(dir: &Path) -> Result<(), SandboxError> {
    for entry in fs::read_dir(dir).map_err(ctx("read_dir", dir))? {
        let path = entry.map_err(ctx("read_dir entry in", dir))?.path();
        let meta = fs::symlink_metadata(&path).map_err(ctx("symlink_metadata", &path))?;
        if is_link(&meta) {
            continue; // never apply ACL changes through a link
        }
        acl::reset_to_inherited(&path)?;
        if meta.is_dir() {
            reset_tree(&path)?;
        }
    }
    Ok(())
}

fn remove_with_retry(dir: &Path) -> io::Result<()> {
    let mut last = None;
    for _ in 0..DELETE_ATTEMPTS {
        // std's remove_dir_all does not follow links and clears read-only attributes.
        match fs::remove_dir_all(dir) {
            Ok(()) => return Ok(()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(()),
            Err(e) => last = Some(e), // e.g. a dying process still holds a handle
        }
        thread::sleep(DELETE_BACKOFF);
    }
    Err(last.unwrap_or_else(|| io::Error::other("remove_dir_all failed")))
}
