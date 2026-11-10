//! Grant the AppContainer SID access to exactly the folders it needs.
//!
//! An AppContainer can open a file only if the DACL allows its SID (or ALL APPLICATION
//! PACKAGES, which LPAC drops). We add one inheritable allow entry per granted folder.

use std::path::Path;
use std::ptr::null_mut;
use std::sync::Mutex;

use windows_sys::Win32::Foundation::LocalFree;
use windows_sys::Win32::Security::Authorization::{
    EXPLICIT_ACCESS_W, GRANT_ACCESS, GetNamedSecurityInfoW, NO_MULTIPLE_TRUSTEE, REVOKE_ACCESS,
    SE_FILE_OBJECT, SE_OBJECT_TYPE, SE_REGISTRY_KEY, SetEntriesInAclW, SetNamedSecurityInfoW,
    TRUSTEE_IS_SID, TRUSTEE_IS_UNKNOWN, TRUSTEE_W,
};
use windows_sys::Win32::Security::{
    ACCESS_ALLOWED_ACE, ACL, ACL_REVISION, CONTAINER_INHERIT_ACE, DACL_SECURITY_INFORMATION,
    EqualSid, GetAce, INHERITED_ACE, InitializeAcl, NO_INHERITANCE, OBJECT_INHERIT_ACE,
    PSECURITY_DESCRIPTOR, PSID, SUB_CONTAINERS_AND_OBJECTS_INHERIT,
    UNPROTECTED_DACL_SECURITY_INFORMATION,
};
use windows_sys::Win32::Storage::FileSystem::{
    DELETE, FILE_GENERIC_EXECUTE, FILE_GENERIC_READ, FILE_GENERIC_WRITE,
};
use windows_sys::Win32::System::SystemServices::ACCESS_ALLOWED_ACE_TYPE;

use super::appcontainer::AppContainerSid;
use super::{last_error, wide};
use crate::SandboxError;

#[derive(Clone, Copy)]
pub(crate) enum Access {
    ReadExecute,
    ReadWriteExecute,
}

impl Access {
    fn mask(self) -> u32 {
        match self {
            Access::ReadExecute => FILE_GENERIC_READ | FILE_GENERIC_EXECUTE,
            Access::ReadWriteExecute => {
                FILE_GENERIC_READ | FILE_GENERIC_EXECUTE | FILE_GENERIC_WRITE | DELETE
            }
        }
    }
}

/// Serializes DACL rewrites so concurrent runs don't race on the same folder.
static ACL_LOCK: Mutex<()> = Mutex::new(());

/// Owns a buffer returned by a Win32 API that must be released with LocalFree.
struct LocalBox(*mut core::ffi::c_void);
impl Drop for LocalBox {
    fn drop(&mut self) {
        if !self.0.is_null() {
            // SAFETY: the pointer came from an API documented to need LocalFree.
            unsafe { LocalFree(self.0) };
        }
    }
}

fn trustee(sid: &AppContainerSid) -> TRUSTEE_W {
    TRUSTEE_W {
        pMultipleTrustee: null_mut(),
        MultipleTrusteeOperation: NO_MULTIPLE_TRUSTEE,
        TrusteeForm: TRUSTEE_IS_SID,
        TrusteeType: TRUSTEE_IS_UNKNOWN,
        ptstrName: sid.as_psid() as *mut u16,
    }
}

fn read_dacl(wpath: &[u16], kind: SE_OBJECT_TYPE) -> Result<(*mut ACL, LocalBox), SandboxError> {
    let mut dacl: *mut ACL = null_mut();
    let mut sd: PSECURITY_DESCRIPTOR = null_mut();
    // SAFETY: wpath is NUL-terminated; on success `sd` owns the descriptor that `dacl`
    // points into, and we keep it alive via LocalBox.
    let err = unsafe {
        GetNamedSecurityInfoW(
            wpath.as_ptr(),
            kind,
            DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            &mut dacl,
            null_mut(),
            &mut sd,
        )
    };
    if err != 0 {
        return Err(SandboxError::Win32 {
            op: "GetNamedSecurityInfoW",
            code: err,
        });
    }
    Ok((dacl, LocalBox(sd)))
}

/// True if `dacl` already has an explicit, inheritable allow entry for `sid` covering `mask`.
fn already_granted(dacl: *const ACL, sid: &AppContainerSid, mask: u32) -> bool {
    if dacl.is_null() {
        return false;
    }
    let want_flags = (CONTAINER_INHERIT_ACE | OBJECT_INHERIT_ACE) as u8;
    // SAFETY: `dacl` is a valid ACL from GetNamedSecurityInfoW; GetAce returns pointers
    // into it for indices < AceCount, and ACCESS_ALLOWED_ACE is the layout for type 0.
    unsafe {
        for i in 0..u32::from((*dacl).AceCount) {
            let mut ace: *mut core::ffi::c_void = null_mut();
            if GetAce(dacl, i, &mut ace) == 0 {
                continue;
            }
            let ace = ace as *const ACCESS_ALLOWED_ACE;
            let h = (*ace).Header;
            if u32::from(h.AceType) == ACCESS_ALLOWED_ACE_TYPE
                && h.AceFlags & INHERITED_ACE as u8 == 0
                && h.AceFlags & want_flags == want_flags
                && (*ace).Mask & mask == mask
                && EqualSid(&(*ace).SidStart as *const u32 as PSID, sid.as_psid()) != 0
            {
                return true;
            }
        }
    }
    false
}

fn apply(
    wpath: &[u16],
    kind: SE_OBJECT_TYPE,
    ea: &EXPLICIT_ACCESS_W,
    old: *const ACL,
) -> Result<(), SandboxError> {
    let mut new_acl: *mut ACL = null_mut();
    // SAFETY: `ea` and `old` are valid for the call; `new_acl` is freed with LocalFree.
    let err = unsafe { SetEntriesInAclW(1, ea, old, &mut new_acl) };
    if err != 0 {
        return Err(SandboxError::Win32 {
            op: "SetEntriesInAclW",
            code: err,
        });
    }
    let _guard = LocalBox(new_acl as *mut _);
    // SAFETY: wpath is NUL-terminated; new_acl is a valid ACL. This also propagates
    // inheritable entries to existing children.
    let err = unsafe {
        SetNamedSecurityInfoW(
            wpath.as_ptr(),
            kind,
            DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            new_acl,
            std::ptr::null(),
        )
    };
    if err != 0 {
        return Err(SandboxError::Win32 {
            op: "SetNamedSecurityInfoW",
            code: err,
        });
    }
    Ok(())
}

/// Give the AppContainer `access` to `path` and everything below it. Idempotent.
pub(crate) fn grant(
    path: &Path,
    sid: &AppContainerSid,
    access: Access,
) -> Result<(), SandboxError> {
    let _lock = ACL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let wpath = wide(path);
    let (dacl, _sd) = read_dacl(&wpath, SE_FILE_OBJECT)?;
    if already_granted(dacl, sid, access.mask()) {
        return Ok(());
    }
    let ea = EXPLICIT_ACCESS_W {
        grfAccessPermissions: access.mask(),
        grfAccessMode: GRANT_ACCESS,
        grfInheritance: SUB_CONTAINERS_AND_OBJECTS_INHERIT,
        Trustee: trustee(sid),
    };
    apply(&wpath, SE_FILE_OBJECT, &ea, dacl)
}

/// Drop every explicit entry on `path` and turn inheritance back on, so it carries only
/// what its parent passes down. Undoes anything the sandbox set on its own files.
/// `path` must not be a link: Windows would apply this to the link target.
pub(crate) fn reset_to_inherited(path: &Path) -> Result<(), SandboxError> {
    let wpath = wide(path);
    // An ACL with no entries: header only, 8 bytes, DWORD-aligned via u64 storage.
    let mut buf = [0u64; 1];
    let empty = buf.as_mut_ptr() as *mut ACL;
    // SAFETY: `buf` is exactly size_of::<ACL>() bytes and suitably aligned.
    if unsafe { InitializeAcl(empty, size_of::<ACL>() as u32, ACL_REVISION) } == 0 {
        return Err(last_error("InitializeAcl"));
    }
    let _lock = ACL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    // SAFETY: NUL-terminated path; `empty` is a valid ACL that outlives the call.
    let err = unsafe {
        SetNamedSecurityInfoW(
            wpath.as_ptr(),
            SE_FILE_OBJECT,
            DACL_SECURITY_INFORMATION | UNPROTECTED_DACL_SECURITY_INFORMATION,
            null_mut(),
            null_mut(),
            empty,
            std::ptr::null(),
        )
    };
    if err != 0 {
        return Err(SandboxError::Win32 {
            op: "SetNamedSecurityInfoW (reset)",
            code: err,
        });
    }
    Ok(())
}

/// Remove every explicit entry for the AppContainer SID from `path`. Missing paths are ignored.
pub(crate) fn revoke(path: &Path, sid: &AppContainerSid) -> Result<(), SandboxError> {
    if !path.exists() {
        return Ok(());
    }
    revoke_object(&wide(path), SE_FILE_OBJECT, sid)
}

/// Remove every explicit entry for the AppContainer SID from a registry key, given in
/// `GetNamedSecurityInfoW` form (e.g. `CURRENT_USER\Software\...`).
pub(crate) fn revoke_registry(key: &str, sid: &AppContainerSid) -> Result<(), SandboxError> {
    revoke_object(&wide(key), SE_REGISTRY_KEY, sid)
}

fn revoke_object(
    wpath: &[u16],
    kind: SE_OBJECT_TYPE,
    sid: &AppContainerSid,
) -> Result<(), SandboxError> {
    let _lock = ACL_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let (dacl, _sd) = read_dacl(wpath, kind)?;
    if !has_explicit_ace(dacl, sid) {
        return Ok(());
    }
    let ea = EXPLICIT_ACCESS_W {
        grfAccessPermissions: 0,
        grfAccessMode: REVOKE_ACCESS,
        grfInheritance: NO_INHERITANCE,
        Trustee: trustee(sid),
    };
    apply(wpath, kind, &ea, dacl)
}

/// True if `dacl` has any non-inherited entry (of any type) for `sid`.
fn has_explicit_ace(dacl: *const ACL, sid: &AppContainerSid) -> bool {
    if dacl.is_null() {
        return false;
    }
    // SAFETY: as in `already_granted`; allow and deny ACEs share the header/mask/SID layout.
    unsafe {
        for i in 0..u32::from((*dacl).AceCount) {
            let mut ace: *mut core::ffi::c_void = null_mut();
            if GetAce(dacl, i, &mut ace) == 0 {
                continue;
            }
            let ace = ace as *const ACCESS_ALLOWED_ACE;
            if (*ace).Header.AceFlags & INHERITED_ACE as u8 == 0
                && EqualSid(&(*ace).SidStart as *const u32 as PSID, sid.as_psid()) != 0
            {
                return true;
            }
        }
    }
    false
}
