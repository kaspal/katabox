//! What the sandbox may touch: host files, the seccomp deny-list, Landlock rules and
//! rlimits. Shared by both modes; differences are explicit (`mode == Fallback`).

use std::path::{Path, PathBuf};

use hakoniwa::landlock::{CompatMode, FsAccess, Resource, Ruleset};
use hakoniwa::seccomp::{Action, ArgCmp, ArgCmpOp, Filter};

use crate::SandboxMode;

const EPERM: i32 = 1;
const ENOSYS: i32 = 38;

/// glibc core libraries the pinned runtimes link against (checked with `ldd`).
const GLIBC_LIBS: &[&str] = &[
    "libc.so.6",
    "libm.so.6",
    "libpthread.so.0",
    "libdl.so.2",
    "libutil.so.1",
    "librt.so.1",
];

#[cfg(target_arch = "x86_64")]
const LIB_DIRS: &[&str] = &[
    "/lib/x86_64-linux-gnu",
    "/usr/lib/x86_64-linux-gnu",
    "/lib64",
    "/usr/lib64",
];
#[cfg(target_arch = "x86_64")]
const INTERPRETER: &str = "/lib64/ld-linux-x86-64.so.2";

#[cfg(target_arch = "aarch64")]
const LIB_DIRS: &[&str] = &[
    "/lib/aarch64-linux-gnu",
    "/usr/lib/aarch64-linux-gnu",
    "/lib64",
    "/usr/lib64",
];
#[cfg(target_arch = "aarch64")]
const INTERPRETER: &str = "/lib/ld-linux-aarch64.so.1";

/// The dynamic loader plus each glibc core library, at the first path the loader
/// searches. These are the only host files outside `read_only_dirs` the sandbox can read
/// (recorded as a known gap, like the Windows system-file gap).
pub(crate) fn host_files() -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();
    if !Path::new(INTERPRETER).is_file() {
        return Err(format!("dynamic loader {INTERPRETER} not found"));
    }
    files.push(PathBuf::from(INTERPRETER));
    for lib in GLIBC_LIBS {
        let found = LIB_DIRS
            .iter()
            .map(|d| Path::new(d).join(lib))
            .find(|p| p.is_file())
            .ok_or_else(|| format!("{lib} not found in {LIB_DIRS:?}"))?;
        files.push(found);
    }
    Ok(files)
}

/// Device files the program may use. `/dev/tty` is deliberately absent.
pub(crate) const DEVICES_RW: &[&str] = &["/dev/null"];
pub(crate) const DEVICES_RO: &[&str] = &["/dev/zero", "/dev/urandom", "/dev/random"];

pub(crate) fn landlock(
    mode: SandboxMode,
    read_only_dirs: &[PathBuf],
    work_dir: &Path,
    host_files: &[PathBuf],
) -> Ruleset {
    let mut r = Ruleset::default();
    r.restrict(Resource::FS, CompatMode::Enforce);
    for d in read_only_dirs {
        r.allow_path(&d.to_string_lossy(), FsAccess::R | FsAccess::X);
    }
    r.allow_path(
        &work_dir.to_string_lossy(),
        FsAccess::R | FsAccess::W | FsAccess::X,
    );
    for f in host_files {
        r.allow_path(&f.to_string_lossy(), FsAccess::R | FsAccess::X);
    }
    for d in DEVICES_RW {
        r.allow_path(d, FsAccess::R | FsAccess::W);
    }
    for d in DEVICES_RO {
        r.allow_path(d, FsAccess::R);
    }
    if mode == SandboxMode::Namespaces {
        // The namespace's own /proc: only sandbox processes are visible in it.
        r.allow_path("/proc", FsAccess::R);
    }
    r
}

/// Syscalls no user program needs and that widen the kernel attack surface or reach
/// outside the sandbox. Fail with EPERM so attacks are observable.
const DENY: &[&str] = &[
    // Debugging and other processes' memory.
    "ptrace",
    "process_vm_readv",
    "process_vm_writev",
    "process_madvise",
    "pidfd_getfd",
    "kcmp",
    "perf_event_open",
    "userfaultfd",
    // Mounts and namespaces.
    "mount",
    "umount2",
    "pivot_root",
    "chroot",
    "move_mount",
    "open_tree",
    "fsopen",
    "fsmount",
    "fsconfig",
    "fspick",
    "mount_setattr",
    "unshare",
    "setns",
    // Kernel, keyring, modules, BPF.
    "bpf",
    "keyctl",
    "add_key",
    "request_key",
    "kexec_load",
    "kexec_file_load",
    "init_module",
    "finit_module",
    "delete_module",
    "io_uring_setup",
    "io_uring_enter",
    "io_uring_register",
    // System administration.
    "reboot",
    "swapon",
    "swapoff",
    "acct",
    "quotactl",
    "quotactl_fd",
    "syslog",
    "settimeofday",
    "clock_settime",
    "clock_adjtime",
    "adjtimex",
    "sethostname",
    "setdomainname",
    "vhangup",
    "lookup_dcookie",
    "uselib",
    "nfsservctl",
    "fanotify_init",
    "open_by_handle_at",
    "name_to_handle_at",
    "iopl",
    "ioperm",
    "modify_ldt",
    // Signals through a pidfd (no pid ≤ 0 form exists; denied outright).
    "pidfd_send_signal",
];

/// Fallback only. Host-wide IPC that Landlock cannot see (names, not paths; the IPC
/// namespace covers this in namespace mode), sockets, and metadata changes: Landlock has
/// no right for chmod/chown/xattrs, and without a read-only bind mount the sandbox could
/// otherwise change the mode of the user's own toolchain files (e.g. make them
/// world-writable).
const DENY_FALLBACK: &[&str] = &[
    "socket",
    "chmod",
    "fchmod",
    "fchmodat",
    "fchmodat2",
    "chown",
    "fchown",
    "lchown",
    "fchownat",
    "setxattr",
    "lsetxattr",
    "fsetxattr",
    "setxattrat",
    "removexattr",
    "lremovexattr",
    "fremovexattr",
    "removexattrat",
    "shmget",
    "shmat",
    "shmctl",
    "shmdt",
    "msgget",
    "msgsnd",
    "msgrcv",
    "msgctl",
    "semget",
    "semop",
    "semtimedop",
    "semctl",
    "mq_open",
    "mq_unlink",
    "mq_timedsend",
    "mq_timedreceive",
    "mq_notify",
    "mq_getsetattr",
];

const AF_UNIX: u64 = 1;
const CLONE_NEW_FLAGS: &[u64] = &[
    0x0002_0000, // CLONE_NEWNS
    0x0200_0000, // CLONE_NEWCGROUP
    0x0400_0000, // CLONE_NEWUTS
    0x0800_0000, // CLONE_NEWIPC
    0x1000_0000, // CLONE_NEWUSER
    0x2000_0000, // CLONE_NEWPID
    0x4000_0000, // CLONE_NEWNET
];
const TIOCSTI: u64 = 0x5412;
const TIOCLINUX: u64 = 0x541C;

fn masked(arg: u32, mask: u64, value: u64) -> ArgCmp {
    ArgCmp::new(arg, ArgCmpOp::MaskedEq, mask, value)
}

pub(crate) fn seccomp(mode: SandboxMode) -> Filter {
    let deny = Action::Errno(EPERM);
    // Default allow; only the native architecture is added, so syscalls made through
    // another ABI (e.g. int 0x80 on x86_64) hit libseccomp's bad-arch action and kill.
    let mut f = Filter::new(Action::Allow);
    for name in DENY {
        f.add_rule(deny, name);
    }

    // Signals to pid ≤ 0 (process groups, every process with kill(-1)). The kernel reads
    // the pid as a 32-bit int, so test the low 32 bits: zero, or the sign bit set.
    for name in [
        "kill",
        "tkill",
        "tgkill",
        "rt_sigqueueinfo",
        "rt_tgsigqueueinfo",
    ] {
        f.add_rule_conditional(deny, name, &[masked(0, 0xFFFF_FFFF, 0)]);
        f.add_rule_conditional(deny, name, &[masked(0, 0x8000_0000, 0x8000_0000)]);
    }

    // No new namespaces through clone flags; clone3 passes flags in memory seccomp
    // cannot read, so make it look unimplemented and glibc falls back to clone.
    for flag in CLONE_NEW_FLAGS {
        f.add_rule_conditional(deny, "clone", &[masked(0, *flag, *flag)]);
    }
    f.add_rule(Action::Errno(ENOSYS), "clone3");

    // Typing into a terminal.
    for req in [TIOCSTI, TIOCLINUX] {
        f.add_rule_conditional(deny, "ioctl", &[masked(1, 0xFFFF_FFFF, req)]);
    }

    // Changing another process's rlimits (e.g. the supervisor's).
    f.add_rule_conditional(deny, "prlimit64", &[ArgCmp::new(0, ArgCmpOp::Ne, 0, 0)]);

    match mode {
        SandboxMode::Fallback => {
            // No sockets at all: Landlock ABI < 7 does not control connecting to a
            // pathname Unix socket (e.g. the user's D-Bus session bus).
            for name in DENY_FALLBACK {
                f.add_rule(deny, name);
            }
        }
        _ => {
            // The network namespace has no interfaces up; refuse anything but Unix
            // sockets anyway, so a misconfigured namespace still has no network.
            f.add_rule_conditional(deny, "socket", &[ArgCmp::new(0, ArgCmpOp::Ne, AF_UNIX, 0)]);
        }
    }
    f
}

/// Per-file size cap (RLIMIT_FSIZE), so a run cannot fill the disk with one file.
pub(crate) const MAX_FILE_BYTES: u64 = 256 * 1024 * 1024;
/// Open file descriptors per process.
pub(crate) const MAX_OPEN_FILES: u64 = 256;
