//! Sandboxed execution of user code.
//!
//! This is the only crate allowed to spawn user programs. Any change here needs
//! explicit maintainer sign-off (see CLAUDE.md).
//!
//! Planned layout:
//! - `windows/`: Job Object (memory, CPU, process count, kill-on-close) + AppContainer
//!   with zero capabilities (no network), ACL grants for toolchain and temp folders.
//! - `linux/`: hakoniwa (user/mount/PID/net namespaces, Landlock, seccomp, rlimits);
//!   fallback to Landlock + seccomp + rlimits when namespaces are unavailable.
//! - `pipeline`: materialize workspace, compile if needed, run harness, stream results.
