//! "Copy diagnostics" (Settings → About; features-content M1): what a bug report needs,
//! and nothing personal. No code, no file paths, no data folder or sandbox root, no user
//! name: only versions, flags and which languages are installed. The UI adds the recent
//! error codes it saw and shows the whole text before copying it.

use kb_core::Language;
use serde::Serialize;

use crate::{App, Result, RootSource};

/// One judge language: whether its runtime is installed, and the pinned version folder.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct RuntimeDiag {
    pub language: Language,
    pub installed: bool,
    /// The pinned runtime, e.g. `python-3.14` (the toolchain folder's name, no path).
    pub version: Option<String>,
    pub language_server: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct Diagnostics {
    pub app_version: String,
    /// `windows`, `linux`, ...
    pub os: String,
    /// `x86_64`, `aarch64`, ...
    pub arch: String,
    pub dev_data: bool,
    /// `<pack id> <pack version>`.
    pub content_pack: String,
    pub user_schema_version: i64,
    /// Where the sandbox root came from (never the path itself).
    pub sandbox_root_source: RootSource,
    /// This machine's measured speed factor (ADR 0017), if measured.
    pub time_scale: Option<f64>,
    pub runtimes: Vec<RuntimeDiag>,
    /// Optional app features that are on (`updater`); the desktop shell fills it in.
    pub features: Vec<String>,
}

/// The version folder of `language`'s runtime in the built-in manifest (`python-3.14`).
fn pinned_runtime(language: Language) -> Option<String> {
    let manifest = kb_toolchain::Manifest::builtin();
    let platform = kb_toolchain::Platform::current()?;
    let unit = manifest.language(language)?;
    let first = unit.runtime.first()?;
    let component = manifest.component(first, platform)?;
    component.dest.rsplit('/').next().map(str::to_string)
}

impl App {
    pub async fn diagnostics(&self) -> Result<Diagnostics> {
        let meta = self.store.content.meta().await?;
        let runtimes = kb_judge::languages::SUPPORTED
            .into_iter()
            .map(|language| RuntimeDiag {
                language,
                installed: self.toolchains.runtime_installed(language),
                version: pinned_runtime(language),
                language_server: self.lsp.unavailable_reason(language).is_none(),
            })
            .collect();
        Ok(Diagnostics {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            dev_data: self.dev_data,
            content_pack: format!("{} {}", meta.pack_id, meta.pack_version),
            user_schema_version: self.store.user.schema_version().await?,
            sandbox_root_source: self.root_source,
            time_scale: self.calibration().map(|c| c.applied),
            runtimes,
            features: Vec::new(),
        })
    }
}
