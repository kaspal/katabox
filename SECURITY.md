# Security policy

KataBox runs code on the user's machine: the user's own solutions, and the problem scripts that ship with the app. It runs all of it inside an OS sandbox. If you find a way around the sandbox or the other protections below, please report it privately.

## Supported versions

| Version | Supported |
| --- | --- |
| 1.0.0-beta.1 | Yes |
| Earlier development builds | No |

Fixes ship in a new release; there is no automatic update, so users install it themselves.

## How KataBox protects your machine

### The sandbox

Every run (Run, Submit, custom tests, the debugger, the complexity profiler, the shrinker) is an ordinary sandboxed process with limits on CPU time (user + kernel), wall-clock time, memory, number of processes and output size. Runs never inherit the host environment, and only one sandboxed run is active at a time.

**Windows** ([ADR 0006](docs/adr/0006-windows-sandbox-details.md)):

- an AppContainer (`KataBox.Sandbox`) with **no capabilities**, so no network access;
- a Job Object per run: memory, CPU and process caps, kill-on-close, no breakaway;
- file access by explicit grants only: read-only on the installed language, read-write on a fresh work folder that exists only for that run and is deleted afterwards;
- every path is checked to be inside KataBox's sandbox root; drive roots, network paths and the user profile are refused, and links are never followed during cleanup.

**Linux** ([ADR 0007](docs/adr/0007-linux-sandbox-details.md)): namespaces (user, mount, PID, network, IPC, UTS), Landlock and a seccomp deny-list, with a Landlock + seccomp fallback where user namespaces are blocked. **Linux builds are not released yet.**

### Language downloads

Runtimes and language servers are downloaded only when the user installs a language ([ADR 0015](docs/adr/0015-toolchain-manager.md)):

- HTTPS only, to an allow-list of hosts compiled into the app (see `crates/kb-toolchain/manifest.toml`); redirects must stay on the list;
- every file is pinned by URL, size and SHA-256 (npm packages also by their sha512 integrity) and verified before extraction;
- archives are extracted by KataBox's own code: no absolute paths, `..`, links, devices or duplicate names, with a size cap; nothing is executed during an install.

### Language servers

Language servers (autocomplete and diagnostics) run outside the sandbox, so they are configured never to execute user code: no build scripts, proc-macros, `cargo`, interpreters or type acquisition ([ADR 0014](docs/adr/0014-language-servers.md)).

### The app

- The UI talks to Rust only through typed commands; it cannot read files or start processes.
- The webview's Content Security Policy allows only the app's own resources and its IPC channel (`connect-src ipc: http://ipc.localhost`).
- Import files are size-capped and parsed strictly (exact format, no unknown fields, value checks); importing never executes anything.

## In scope

- **Sandbox escapes:** code run by KataBox that reaches the network, reads or writes files outside its work folder and the installed language, affects other processes, survives the run, or exhausts the machine despite the limits.
- **Download verification:** a way to make KataBox install a file other than the pinned one, contact a host outside the allow-list, or write outside the sandbox root while extracting.
- **Language servers:** a way for problem or user code to be executed by a language server.
- **IPC and CSP:** a way for content shown in the app (problem statements, editorials, Learn topics, program output) to run script, call commands it should not, or load remote resources.
- **Import files:** a crafted export file that crashes the app, corrupts data, writes outside `user.db` or executes anything.
- **Installer and uninstaller:** for example cleanup that touches files outside KataBox's folders.

## Out of scope

- **Hidden tests are not secret.** They ship as plain data with the app, by design ([ADR 0005](docs/adr/0005-open-source-and-privacy.md)); there is no anti-cheat.
- Attacks that need another process already running as the same user: it can change the app and its folders anyway.
- Known, recorded limitations: on Windows the sandbox can read files Windows grants to all AppContainers (much of `C:\Windows` and `Program Files`).
- Unsigned installers (SmartScreen warnings) during the beta.

## Reporting a vulnerability

**Do not open a public issue.**

- Contact: `[MAINTAINER: contact]`
- Once the repository is public, you can also use GitHub's private vulnerability reporting ("Report a vulnerability" under the repository's Security tab).

Please include the KataBox version, your Windows version, what you did (a minimal solution file or import file helps) and what happened. Copy diagnostics (Settings) gives the versions without any personal data.

We aim to acknowledge reports within 7 days.
