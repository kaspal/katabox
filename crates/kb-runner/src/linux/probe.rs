//! What this kernel and system support. Probed once per process and cached.

use std::path::Path;
use std::process::{Command, ExitCode, Stdio};
use std::sync::OnceLock;

use landlock::{
    ABI, Access, AccessFs, AccessNet, CompatLevel, Compatible, Ruleset, RulesetAttr, Scope,
};
use nix::mount::{MsFlags, mount};
use nix::sched::{CloneFlags, unshare};
use nix::unistd::{AccessFlags, Gid, Uid, access};
use serde::{Deserialize, Serialize};

use crate::SandboxError;

/// Sandbox-relevant facts about the running Linux system (`kb-sandbox probe`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinuxSupport {
    /// `uname -r`.
    pub kernel: String,
    /// Highest Landlock ABI usable (0 = Landlock unavailable). 6 adds signal and
    /// abstract-socket scoping, which fallback mode requires.
    pub landlock_abi: u32,
    /// User + mount + network namespaces with an id map and a mount actually worked.
    pub user_namespaces: bool,
    /// Why the user-namespace trial failed (e.g. Ubuntu's AppArmor restriction).
    pub user_namespaces_error: Option<String>,
    /// seccomp filters available (`Seccomp:` present in /proc/self/status).
    pub seccomp: bool,
    /// The unified cgroup v2 hierarchy is mounted at /sys/fs/cgroup.
    pub cgroup_v2: bool,
    /// This process's cgroup (from /proc/self/cgroup).
    pub cgroup: String,
    /// This process's cgroup folder is writable (delegated to the user).
    pub cgroup_writable: bool,
    /// Controllers enabled for children of this cgroup.
    pub cgroup_subtree_controllers: String,
}

impl LinuxSupport {
    pub fn signal_scoping(&self) -> bool {
        self.landlock_abi >= 6
    }
}

static PROBE: OnceLock<LinuxSupport> = OnceLock::new();

/// Probe once (cached for the life of the process). Needs the helper binary for the
/// user-namespace trial, which must run in a fresh single-threaded process.
pub fn probe() -> Result<LinuxSupport, SandboxError> {
    if let Some(p) = PROBE.get() {
        return Ok(p.clone());
    }
    let helper = super::helper()?;
    let p = probe_with(&helper);
    Ok(PROBE.get_or_init(|| p).clone())
}

fn probe_with(helper: &Path) -> LinuxSupport {
    let kernel = nix::sys::utsname::uname()
        .map(|u| u.release().to_string_lossy().into_owned())
        .unwrap_or_default();
    let (user_namespaces, user_namespaces_error) = match Command::new(helper)
        .arg("__probe-userns")
        .env_clear()
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .output()
    {
        Ok(o) if o.status.success() => (true, None),
        Ok(o) => (
            false,
            Some(String::from_utf8_lossy(&o.stderr).trim().to_string()),
        ),
        Err(e) => (false, Some(format!("could not start probe: {e}"))),
    };
    let seccomp = std::fs::read_to_string("/proc/self/status")
        .is_ok_and(|s| s.lines().any(|l| l.starts_with("Seccomp:")));
    let cgroup_v2 = Path::new("/sys/fs/cgroup/cgroup.controllers").is_file();
    let cgroup = std::fs::read_to_string("/proc/self/cgroup")
        .ok()
        .and_then(|s| {
            s.lines()
                .find_map(|l| l.strip_prefix("0::").map(str::to_string))
        })
        .unwrap_or_default();
    let dir = Path::new("/sys/fs/cgroup").join(cgroup.trim_start_matches('/'));
    let cgroup_writable = cgroup_v2 && access(&dir, AccessFlags::W_OK).is_ok();
    let cgroup_subtree_controllers = std::fs::read_to_string(dir.join("cgroup.subtree_control"))
        .unwrap_or_default()
        .trim()
        .to_string();
    LinuxSupport {
        kernel,
        landlock_abi: landlock_abi(),
        user_namespaces,
        user_namespaces_error,
        seccomp,
        cgroup_v2,
        cgroup,
        cgroup_writable,
        cgroup_subtree_controllers,
    }
}

/// Highest ABI for which a ruleset with that ABI's new feature can be created as a hard
/// requirement. Creating a ruleset restricts nothing; only `restrict_self` would.
fn landlock_abi() -> u32 {
    let hard = || Ruleset::default().set_compatibility(CompatLevel::HardRequirement);
    let ok = |abi: ABI| -> bool {
        let r = match abi {
            ABI::V6 => hard().scope(Scope::from_all(ABI::V6)),
            ABI::V4 => hard().handle_access(AccessNet::from_all(ABI::V4)),
            _ => hard().handle_access(AccessFs::from_all(abi)),
        };
        r.and_then(|r| r.create()).is_ok()
    };
    for (n, abi) in [
        (6, ABI::V6),
        (5, ABI::V5),
        (4, ABI::V4),
        (3, ABI::V3),
        (2, ABI::V2),
        (1, ABI::V1),
    ] {
        if ok(abi) {
            return n;
        }
    }
    0
}

/// `kb-sandbox __probe-userns`: do what namespace mode needs (new user, mount, network,
/// IPC and UTS namespaces, id maps, a mount) in this throwaway process. Ubuntu's AppArmor
/// restriction lets `unshare` succeed and fails the later steps, so all are tried.
pub(crate) fn userns_main() -> ExitCode {
    let uid = Uid::current();
    let gid = Gid::current();
    let step = |what: &str, r: Result<(), String>| -> Result<(), String> {
        r.map_err(|e| format!("{what}: {e}"))
    };
    let result = (|| {
        step(
            "unshare",
            unshare(
                CloneFlags::CLONE_NEWUSER
                    | CloneFlags::CLONE_NEWNS
                    | CloneFlags::CLONE_NEWNET
                    | CloneFlags::CLONE_NEWIPC
                    | CloneFlags::CLONE_NEWUTS,
            )
            .map_err(|e| e.to_string()),
        )?;
        step(
            "setgroups",
            std::fs::write("/proc/self/setgroups", "deny").map_err(|e| e.to_string()),
        )?;
        step(
            "uid_map",
            std::fs::write("/proc/self/uid_map", format!("{uid} {uid} 1"))
                .map_err(|e| e.to_string()),
        )?;
        step(
            "gid_map",
            std::fs::write("/proc/self/gid_map", format!("{gid} {gid} 1"))
                .map_err(|e| e.to_string()),
        )?;
        step(
            "make / private",
            mount(
                None::<&str>,
                "/",
                None::<&str>,
                MsFlags::MS_REC | MsFlags::MS_PRIVATE,
                None::<&str>,
            )
            .map_err(|e| e.to_string()),
        )?;
        step(
            "mount tmpfs",
            mount(
                Some("tmpfs"),
                "/tmp",
                Some("tmpfs"),
                MsFlags::MS_NOSUID | MsFlags::MS_NODEV,
                None::<&str>,
            )
            .map_err(|e| e.to_string()),
        )
    })();
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::from(1)
        }
    }
}
