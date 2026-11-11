# 15. Installing languages: the toolchain manager

Date: 2026-11-11 · Status: proposed

## Context
Until now runtimes and language servers reached a machine only through
`scripts/fetch-toolchains.*`, for developers and CI. Real users need the app to install
them:
- safely;
- pinned per app version;
- one language at a time;
- through a first-run screen and Settings.

Developers and users must share one code path, so that what CI tests is what users
run.

This ADR also covers the network policy the installer follows. It builds on ADR 0002
(native runtimes), 0005 (privacy), 0006/0007 (sandbox roots), 0009 (PHP and Rust) and
0014 (language servers). It supersedes ADR 0002's "Rust via rustup minimal" (already
replaced by ADR 0009) and changes where ADR 0014's rust-src lives.

## Decision

### One manifest, compiled in
- `crates/kb-toolchain/manifest.toml` is compiled into the app. Changing a pin means
  changing that file; nowhere else.
- It lists every **artifact**: URL(s), size, SHA-256, format (`zip` or `tar.gz`), and for
  npm packages also the lockfile's sha512 `integrity`.
- It lists every **component**, each a folder under the sandbox root: its sources, the
  subtree to extract, excludes, links to copy, a check file, and the installed size.
- It lists the **languages** the user picks. Each is a runtime plus a language server:

  | Language | Components |
  | --- | --- |
  | Python | python + node + basedpyright |
  | JS/TS | node + typescript + typescript-language-server |
  | PHP | php |
  | Rust | rust + rust-src + rust-analyzer |

  Node is shared and kept while any installed language needs it. Linux has Python and
  JS/TS only (ADR 0009).
- There is **no separate signing key in v1**: the pins are as trustworthy as the app
  binary that carries them. A signed manifest that can change without an app update is a
  follow-up.
- A pin ships only after the **real-pins test** has downloaded it: every artifact, every
  mirror, through the app's own downloader. The test checks size and SHA-256, then
  installs every language of the platform and checks installed sizes within 1%.
- `scripts/fetch-toolchains.ps1`/`.sh` are thin wrappers around `kb-toolchain install`:
  the same code, the same verification.

### Network policy (ADR 0005)
- **When:** network is used only to install languages, and only on an explicit user
  action: Install or Retry. There are no background checks.
- **Where:** HTTPS only, port 443, no user-info in URLs, and only to the manifest's
  hosts:
  - github.com and GitHub's release-asset hosts;
  - nodejs.org;
  - windows.php.net and downloads.php.net;
  - static.rust-lang.org;
  - registry.npmjs.org.
- **Redirects:** every hop must stay on that list, with at most 5 hops. Anything else is
  `host_not_allowed`.
- **TLS:** rustls with the ring provider, verified against the **OS certificate store**
  (rustls-platform-verifier). That respects enterprise CAs and needs no bundled roots.
- **Proxy:** the system proxy only. PAC/WPAD is not supported, and there is no app proxy
  setting.
- **Privacy:** the User-Agent is `KataBox/<version>`; nothing about the user is sent.

### Downloads
- **No decoding:** responses are never decompressed by the HTTP client, so sizes and hashes
  apply to the exact bytes of the pinned file.
- **Streaming:** each file goes to `<root>/downloads/<sha256>.part`. The download stops as
  soon as it exceeds the pinned size.
- **Resume:**
  - It always starts again from the manifest URL. GitHub's signed redirect targets
    expire, so they are never reused.
  - A `206` is accepted only if `Content-Range` starts at the `.part` length and its total
    equals the pinned size. Anything else restarts from zero, once.
  - A `.part` longer than the pin is deleted. One of exactly the pinned size is only
    verified.
- **Verification:** the whole file's size and SHA-256 are checked (and npm's sha512)
  before it is accepted. A mismatch deletes the file and fails, with no retry loop.
- **Cancel:** it keeps the `.part` file. **Retry** resumes it.
- **Mirrors:** they are tried in order, for transport errors only. PHP lists `releases/`
  and `releases/archives/`, because superseded builds move.

### Extraction (our code; nothing is executed during an install)
- **Same bytes:** the verified file is opened without write sharing (Windows) and hashed
  again from that handle. `tar.gz` is hashed a third time while it streams. A mismatch
  means the staging folder is never promoted.
- **Fresh folder:** files go into a new `<root>/staging/<job>/<n>/`, with short paths. Only
  regular files and folders are written, every file is created new (`create_new`), and
  Unix modes are only 0755 or 0644.
- **Rejected paths (`unsafe_archive`):** absolute paths, roots or drive prefixes; `..`;
  backslashes, NUL or control characters; `<>:"|?*`; Windows device names; trailing dots
  or spaces. This is checked for every entry, on every platform.
- **Rejected entries:** devices and fifos; encrypted or symlink zip entries; duplicates,
  including names that differ only in case.
- **Links:**
  - Hard links and symlinks are rejected unless the manifest lists them.
  - Listed ones (python-build-standalone's `bin/python3` on Linux) are **copied** from a
    regular file of the same archive, resolved inside the extracted tree.
  - Listed skips (terminfo aliases, Node's `bin/npm`) are left out.
  - No link is ever created.
- **Zip-bomb guard:** written bytes are capped at 110% of the expected installed size,
  plus an entry-count cap.
- **Final check:** after extraction there must be no links or reparse points, and the
  component's check file must exist.
- **npm:** npm is never run. The three registry tarballs are verified (sha256 and sha512),
  and their `package/` folders are extracted into `tools/language-servers/node_modules/`.
  A test requires the manifest to match `scripts/language-servers/package-lock.json`
  exactly. Its only other entry is optional `fsevents`, which is macOS-only.

### Promotion, receipts, recovery
- **Whole-folder rename:** every component folder is replaced as a whole, renamed in from
  staging. Nothing is ever moved into an existing folder. kb-runner grants the sandbox
  read access to a toolchain folder lazily, once, on that folder: a file moved into an
  already-granted folder would keep its staging ACL (ADR 0009). A test runs the judge
  before and after such a replacement.
- **Order of steps:**
  1. sync the staged files (Linux `syncfs`);
  2. rename the folder into place (on Linux, only when the destination is absent);
  3. check the component's check file again (antivirus may quarantine);
  4. write the **receipt last**: `<root>/receipts/<component>.json`, via temp file,
     flush and rename.

  This step can't be cancelled.
- **Receipts are never authoritative:** dests, URLs and hashes always come from the
  compiled manifest. A component counts as installed only with a parseable receipt whose
  per-component digest matches the current manifest **and** whose check file exists.
  Otherwise it is "not installed" and gets replaced, for example after an app update
  with new pins.
- **Locking:** an exclusive `<root>/.install.lock` covers every install, uninstall and
  purge, across processes (the app and the CLI). In-process, one job runs at a time.
  Both give `install_busy`.
- **Purge:** it runs at app start and before every job. It clears `staging/` (trash
  included) and partial downloads of artifacts no longer pinned. Receipt-less or outdated
  folders at **known** component destinations are removed. Nothing unknown is touched.
- **Folder safety:** we create `toolchains/`, `tools/`, `staging/`, `downloads/` and
  `receipts/` ourselves, and refuse any that is a link, junction or reparse point. Every
  operation requires the `.katabox-sandbox-root` marker. The CLI creates the marker only
  for the default dev root or with `--create-root`.
- **Busy files:** renames and deletes retry for about 20 s on Windows sharing errors
  (antivirus, indexer). After that the error is `file_locked`.
- **Disk space:** free space is checked up front: downloads + installed size + 10% +
  200 MB. Sizes are reported before anything starts.

### Uninstall per language
1. The app holds the judge lock: refused with `judge_busy` during a Run or Submit.
2. The receipts of the components no other installed language needs go first. The
   language immediately counts as not installed, for the judge and the language-server
   bridge alike.
3. The bridge's servers whose components went are stopped (Windows file locks).
4. The folders are renamed to trash and deleted. A locked folder is left for the next
   purge.

Uninstalling Rust also clears the judge's `cache/rust/`.

### rust-src moves to `tools/rust-src`
std's sources are a component of the Rust unit in `tools/rust-src`, and rust-analyzer's
`rust-project.json` points `sysroot_src` there (ADR 0014). As a result:
- the judge's `toolchains/rust-1.99` stays small, and sandboxed rustc never sees the
  sources;
- the first Rust Run doesn't propagate a sandbox grant over thousands of files;
- nothing is ever added to an installed sysroot.

### The app
- **Source of truth:** kb-app builds the judge's toolchains and the bridge's availability
  from receipts, and rebuilds them after every install or uninstall.
- **Commands:**
  - `languages()`: per language, available, installed, sizes; plus free space;
  - `install_languages(languages, Channel<InstallEvent>)`: plan, then per component:
    queued, downloading, verifying, extracting, installing, installed;
  - `cancel_install()`;
  - `uninstall_language(language)`.
- **UI:**
  - a first-run screen, shown while no language is installed;
  - Settings → Languages, to install and uninstall;
  - an Install action wherever a language is missing: the workspace picker, a banner
    above the editor, the language-server status;
  - Retry after a failure or cancel, which resumes the downloads;
  - the component licenses on the screen.
- **Errors** reach the UI as stable codes, in English and Spanish: `download_failed`,
  `network_unavailable`, `host_not_allowed`, `checksum_mismatch`, `size_mismatch`,
  `unsafe_archive`, `not_enough_space`, `file_locked`, `install_cancelled`,
  `install_busy`, `unsupported_platform`.

### Threat model
- **In scope:**
  - a hostile or compromised network or mirror: TLS, the host allow-list and the pinned
    hashes stop it;
  - hostile archive contents: the extraction rules stop them;
  - interrupted installs: receipts and purge handle them;
  - users or antivirus deleting files: the receipt and check-file rule handles them.
- **Out of scope:** other processes running as the same user. They can already change
  anything in the user's folders, including the app.

## Consequences
- The Rust download is `.tar.gz`, about 1.5× the size of `.tar.xz` (244 MB vs about
  170 MB), in exchange for one decoder and no xz crate.
- PHP's vcruntime140.dll comes from the pinned Python archive, so installing PHP downloads
  it (51 MB) even when Python is installed. One code path, one hash.
- New dependencies: reqwest's rustls backend, rustls, ring (ISC, added to `deny.toml`),
  rustls-platform-verifier, tar, zip, fs4, rustix (Linux). `deny.toml` now checks only
  the shipped targets (Windows x64, Linux x64 and arm64).
- Roots made by the old scripts have no receipts. They are purged and reinstalled once.

## Follow-ups
- A signed manifest (Ed25519) so pins can update without an app release; pack updates
  through the same downloader.
- Linux: PHP and Rust components (ADR 0009), and phpactor (ADR 0014).
- Memory and process caps for language servers (ADR 0014), and running them in a
  long-lived sandbox profile.
