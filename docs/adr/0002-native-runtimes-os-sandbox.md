# 2. Native runtimes inside an OS sandbox

Date: 2026-11-08 · Status: accepted

## Context
User code must run safely and offline, and debuggers (debugpy, js-debug, Xdebug, CodeLLDB) must work.

## Decision
Download pinned, checksummed runtimes on demand (python-build-standalone, Node LTS, PHP, Rust via rustup minimal; Rust uses the windows-gnu target on Windows). Run user code in an OS sandbox: Job Objects + AppContainer on Windows; hakoniwa (namespaces, Landlock, seccomp, rlimits) on Linux with a Landlock + seccomp fallback.

## Consequences
Real language semantics and working debuggers; no Docker or WSL. Sandbox code is security-critical and gets an attack suite in CI. Rejected: WebAssembly-only runtimes (no debuggers, different semantics) and a custom VM.

Note (2026-11-12): socket debug adapters cannot run in the sandbox, which blocks all networking. The debugger replays runs instead; see ADR 0022.
