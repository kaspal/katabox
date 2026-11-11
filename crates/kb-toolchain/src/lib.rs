//! Language runtimes and language servers for KataBox (ADR 0015).
//!
//! Everything the app may download is pinned in `manifest.toml`, compiled into the
//! binary. [`Manager`] installs languages into a sandbox root: it downloads over HTTPS from
//! allow-listed hosts only, verifies size and SHA-256 before anything is extracted,
//! extracts with its own path checks into a staging folder, renames each component into
//! place as a whole, and writes an install receipt last. The app, the `kb-toolchain` CLI
//! and `scripts/fetch-toolchains.*` all use this one code path.
//!
//! Nothing is executed during an install.

mod download;
mod error;
mod extract;
mod install;
pub mod layout;
pub mod manifest;

pub use download::{AllowList, Downloader};
pub use error::{Error, ErrorCode};
pub use install::{
    ComponentPhase, InstallEvent, InstallOptions, LanguageStatus, Manager, PlanItem, Receipt,
    Uninstalled,
};
pub use manifest::{Manifest, Platform};
pub use tokio_util::sync::CancellationToken;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Download and verify one artifact into `dir` (the real-pins test; the app downloads only
/// through [`Manager::install`]).
#[doc(hidden)]
pub async fn fetch_for_tests(
    downloader: &Downloader,
    artifact: &manifest::Artifact,
    dir: &std::path::Path,
    cancel: &CancellationToken,
) -> Result<()> {
    downloader
        .fetch(artifact, dir, cancel, &mut |_, _| {})
        .await
        .map(|_| ())
}
