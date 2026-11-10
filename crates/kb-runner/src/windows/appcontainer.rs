//! AppContainer profile with zero capabilities: no network, no user files, no devices.

use std::path::Path;
use std::ptr::null_mut;

use windows_sys::Win32::Foundation::ERROR_ALREADY_EXISTS;
use windows_sys::Win32::Security::Isolation::{
    CreateAppContainerProfile, DeleteAppContainerProfile, DeriveAppContainerSidFromAppContainerName,
};
use windows_sys::Win32::Security::{CopySid, FreeSid, GetLengthSid, PSID};

use super::{acl, last_error, wide};
use crate::SandboxError;

/// HRESULT_FROM_WIN32(code) for codes below 0x10000.
const fn hresult_from_win32(code: u32) -> i32 {
    ((code & 0xFFFF) | 0x8007_0000) as i32
}
const HR_ALREADY_EXISTS: i32 = hresult_from_win32(ERROR_ALREADY_EXISTS);
const HR_NOT_FOUND: i32 = hresult_from_win32(2); // ERROR_FILE_NOT_FOUND

/// An owned copy of the AppContainer SID (u64 storage keeps it DWORD-aligned).
pub(crate) struct AppContainerSid(Vec<u64>);

impl AppContainerSid {
    pub(crate) fn as_psid(&self) -> PSID {
        self.0.as_ptr() as PSID
    }
}

/// Create the profile (no capabilities) or, if it exists, derive its SID.
///
/// CreateProcess refuses an AppContainer SID whose profile is not registered, so the
/// profile must exist; `seal_profile_storage` then removes the storage it brings.
pub(crate) fn ensure_profile(name: &str) -> Result<AppContainerSid, SandboxError> {
    let wname = wide(name);
    let wdesc = wide("KataBox sandbox for running user code");
    let mut sid: PSID = null_mut();
    // SAFETY: all strings are NUL-terminated and outlive the call; a null capability
    // array with count 0 means "no capabilities"; `sid` is released by copy_and_free.
    let hr = unsafe {
        CreateAppContainerProfile(
            wname.as_ptr(),
            wname.as_ptr(),
            wdesc.as_ptr(),
            std::ptr::null(),
            0,
            &mut sid,
        )
    };
    if hr == HR_ALREADY_EXISTS {
        // SAFETY: same as above; derive does not create anything.
        let hr = unsafe { DeriveAppContainerSidFromAppContainerName(wname.as_ptr(), &mut sid) };
        if hr < 0 {
            return Err(SandboxError::HResult {
                op: "DeriveAppContainerSidFromAppContainerName",
                hr: hr as u32,
            });
        }
    } else if hr < 0 {
        return Err(SandboxError::HResult {
            op: "CreateAppContainerProfile",
            hr: hr as u32,
        });
    }
    // SAFETY: `sid` was just returned by one of the two APIs above.
    unsafe { copy_and_free(sid) }
}

/// Copy a SID allocated by the AppContainer APIs into owned memory and free the original.
///
/// # Safety
/// `sid` must be a valid SID that must be released with FreeSid.
unsafe fn copy_and_free(sid: PSID) -> Result<AppContainerSid, SandboxError> {
    // SAFETY: per the contract; the buffer holds at least GetLengthSid bytes.
    unsafe {
        let len = GetLengthSid(sid);
        let mut buf = vec![0u64; (len as usize).div_ceil(8)];
        let ok = CopySid(len, buf.as_mut_ptr() as PSID, sid);
        FreeSid(sid);
        if ok == 0 {
            return Err(last_error("CopySid"));
        }
        Ok(AppContainerSid(buf))
    }
}

/// A profile gives its AppContainer a private folder (`%LOCALAPPDATA%\Packages\<name>\AC`,
/// where Windows also points TEMP/TMP) and a private registry key, both writable and
/// both persistent across runs. Revoke the SID's access to both so the only writable
/// place is the per-run work folder. Fails closed if either location is missing.
pub(crate) fn seal_profile_storage(name: &str, sid: &AppContainerSid) -> Result<(), SandboxError> {
    let moniker = name.to_lowercase();
    let local = std::env::var_os("LOCALAPPDATA")
        .ok_or_else(|| SandboxError::InvalidSpec("LOCALAPPDATA is not set".into()))?;
    let folder = Path::new(&local).join("Packages").join(&moniker).join("AC");
    if !folder.is_dir() {
        return Err(SandboxError::InvalidSpec(format!(
            "AppContainer profile folder not found: {}",
            folder.display()
        )));
    }
    acl::revoke(&folder, sid)?;
    acl::revoke_registry(
        &format!(
            r"CURRENT_USER\Software\Classes\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppContainer\Storage\{moniker}"
        ),
        sid,
    )
}

pub(crate) fn delete_profile(name: &str) -> Result<(), SandboxError> {
    let wname = wide(name);
    // SAFETY: NUL-terminated string that outlives the call.
    let hr = unsafe { DeleteAppContainerProfile(wname.as_ptr()) };
    if hr < 0 && hr != HR_NOT_FOUND {
        return Err(SandboxError::HResult {
            op: "DeleteAppContainerProfile",
            hr: hr as u32,
        });
    }
    Ok(())
}
