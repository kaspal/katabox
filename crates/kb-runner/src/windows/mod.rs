//! Windows sandbox: Job Object + AppContainer (optionally Less Privileged).
//!
//! `unsafe` is allowed in this module tree only, because every Win32 call needs it.
//! Each `unsafe` block states why it is sound.
#![allow(unsafe_code)]

mod acl;
mod appcontainer;
mod job;
mod process;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;

use crate::{APP_CONTAINER_NAME, SandboxError};

pub(crate) use process::run;

pub(crate) fn cleanup(dirs: &[PathBuf]) -> Result<(), SandboxError> {
    let sid = appcontainer::ensure_profile(APP_CONTAINER_NAME)?;
    for dir in dirs {
        acl::revoke(dir, &sid)?;
    }
    appcontainer::delete_profile(APP_CONTAINER_NAME)
}

/// NUL-terminated UTF-16 for Win32 `PCWSTR` parameters.
fn wide(s: impl AsRef<OsStr>) -> Vec<u16> {
    s.as_ref().encode_wide().chain(std::iter::once(0)).collect()
}

fn last_error(op: &'static str) -> SandboxError {
    // SAFETY: GetLastError only reads thread-local state.
    let code = unsafe { windows_sys::Win32::Foundation::GetLastError() };
    SandboxError::Win32 { op, code }
}
