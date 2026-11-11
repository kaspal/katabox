//! How each language server is started (ADR 0014): program, arguments, a clean
//! environment, the working folder, and the settings that keep it from ever running
//! user code or fetching anything.

use std::path::{Path, PathBuf};

use kb_core::Language;
use serde_json::{Value, json};

use crate::project::Project;

/// Where the runtimes and servers are: `<root>/toolchains/…` and `<root>/tools/…`
/// (the layout `scripts/fetch-toolchains.ps1` creates).
#[derive(Debug, Clone, PartialEq)]
pub struct Toolchains {
    pub root: PathBuf,
}

impl Toolchains {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Toolchains { root: root.into() }
    }
    pub fn tools(&self) -> PathBuf {
        self.root.join("tools")
    }
    pub fn node_dir(&self) -> PathBuf {
        self.root.join("toolchains").join("node-26")
    }
    pub fn php_dir(&self) -> PathBuf {
        self.root.join("toolchains").join("php-8.5")
    }
    pub fn rust_sysroot(&self) -> PathBuf {
        self.root.join("toolchains").join("rust-1.99")
    }
    fn node_modules(&self) -> PathBuf {
        self.tools().join("language-servers").join("node_modules")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerSpec {
    /// Shown in status messages ("basedpyright").
    pub name: &'static str,
    pub program: PathBuf,
    pub args: Vec<String>,
    /// The complete environment (the process starts with an empty one).
    pub env: Vec<(String, String)>,
    pub cwd: PathBuf,
    /// initializationOptions and workspace/configuration answers.
    pub settings: Value,
    /// The server only serves diagnostics on request (`textDocument/diagnostic`); the
    /// bridge pulls them after edits and hands them to the UI as `publishDiagnostics`,
    /// which is what the editor's client understands.
    pub pull_diagnostics: bool,
}

fn exe(name: &str) -> String {
    if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    }
}

/// Why `language` has no server here, or `None` when its files are present.
pub fn unavailable_reason(language: Language, tc: &Toolchains) -> Option<String> {
    if language == Language::Php && cfg!(windows) {
        return Some(
            "phpactor needs the PHP posix extension, which Windows builds of PHP do not provide"
                .into(),
        );
    }
    let needed: Vec<PathBuf> = match language {
        Language::Python => vec![
            tc.node_dir().join(exe("node")),
            tc.node_modules()
                .join("basedpyright")
                .join("langserver.index.js"),
        ],
        Language::TypeScript | Language::JavaScript => vec![
            tc.node_dir().join(exe("node")),
            tc.node_modules()
                .join("typescript-language-server")
                .join("lib")
                .join("cli.mjs"),
            tc.node_modules()
                .join("typescript")
                .join("lib")
                .join("tsserver.js"),
        ],
        Language::Php => vec![
            tc.php_dir().join(exe("php")),
            tc.tools().join("phpactor").join("phpactor.phar"),
        ],
        Language::Rust => vec![
            tc.tools()
                .join("rust-analyzer")
                .join("bin")
                .join(exe("rust-analyzer")),
            tc.rust_sysroot()
                .join("lib")
                .join("rustlib")
                .join("src")
                .join("rust")
                .join("library"),
        ],
    };
    needed
        .iter()
        .find(|p| !p.exists())
        .map(|p| format!("not installed ({} is missing)", p.display()))
}

/// The environment every server gets: nothing inherited except what Windows needs to
/// run a process at all, and home/temp folders redirected into `state`.
pub fn clean_env(path_dirs: &[PathBuf], state: &Path) -> Vec<(String, String)> {
    let s = |p: &Path| p.display().to_string();
    let mut env = Vec::new();
    if cfg!(windows) {
        for key in ["SystemRoot", "windir", "SystemDrive"] {
            if let Ok(v) = std::env::var(key) {
                env.push((key.to_string(), v));
            }
        }
    }
    let path = std::env::join_paths(path_dirs)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_default();
    env.push(("PATH".into(), path));
    let tmp = state.join("tmp");
    for (k, v) in [
        ("TEMP", tmp.clone()),
        ("TMP", tmp.clone()),
        ("TMPDIR", tmp),
        ("HOME", state.join("home")),
        ("USERPROFILE", state.join("home")),
        ("APPDATA", state.join("appdata")),
        ("LOCALAPPDATA", state.join("localappdata")),
        ("XDG_CONFIG_HOME", state.join("config")),
        ("XDG_CACHE_HOME", state.join("cache")),
        ("XDG_DATA_HOME", state.join("data")),
    ] {
        env.push((k.to_string(), s(&v)));
    }
    env
}

/// The server for `language`, working in `project`, with per-server state in `state`.
pub fn server_for(
    language: Language,
    tc: &Toolchains,
    project: &Project,
    state: &Path,
) -> Result<ServerSpec, String> {
    if let Some(reason) = unavailable_reason(language, tc) {
        return Err(reason);
    }
    for sub in [
        "tmp",
        "home",
        "appdata",
        "localappdata",
        "config",
        "cache",
        "data",
    ] {
        std::fs::create_dir_all(state.join(sub))
            .map_err(|e| format!("cannot create {}: {e}", state.display()))?;
    }
    let s = |p: PathBuf| p.display().to_string();
    let node = tc.node_dir().join(exe("node"));
    let nm = tc.node_modules();
    let spec = match language {
        Language::Python => ServerSpec {
            name: "basedpyright",
            program: node,
            args: vec![
                s(nm.join("basedpyright").join("langserver.index.js")),
                "--stdio".into(),
            ],
            env: clean_env(&[tc.node_dir()], state),
            cwd: project.dir.clone(),
            pull_diagnostics: false,
            // No interpreter is configured: basedpyright uses its bundled typeshed only and
            // never starts Python.
            settings: json!({
                "basedpyright": { "analysis": {
                    "typeCheckingMode": "standard",
                    "diagnosticMode": "openFilesOnly",
                    "autoImportCompletions": false,
                    "autoSearchPaths": false,
                    "useLibraryCodeForTypes": false,
                    "inlayHints": { "variableTypes": false, "callArgumentNames": false, "functionReturnTypes": false }
                } },
                "python": { "analysis": { "diagnosticMode": "openFilesOnly" } }
            }),
        },
        Language::TypeScript | Language::JavaScript => ServerSpec {
            name: "typescript-language-server",
            program: node,
            args: vec![
                s(nm.join("typescript-language-server")
                    .join("lib")
                    .join("cli.mjs")),
                "--stdio".into(),
            ],
            env: clean_env(&[tc.node_dir()], state),
            cwd: project.dir.clone(),
            pull_diagnostics: false,
            // Automatic type acquisition would run npm: off. No plugins.
            settings: json!({
                "disableAutomaticTypingAcquisition": true,
                "tsserver": {
                    "path": s(nm.join("typescript").join("lib").join("tsserver.js")),
                    "logVerbosity": "off",
                    "useSyntaxServer": "never"
                },
                "plugins": [],
                "preferences": { "includeCompletionsForModuleExports": false, "includePackageJsonAutoImports": "off" }
            }),
        },
        Language::Php => ServerSpec {
            name: "phpactor",
            program: tc.php_dir().join(exe("php")),
            args: vec![
                "-n".into(),
                "-d".into(),
                format!("extension_dir={}", s(tc.php_dir().join("ext"))),
                "-d".into(),
                "extension=mbstring".into(),
                "-d".into(),
                "memory_limit=1G".into(),
                s(tc.tools().join("phpactor").join("phpactor.phar")),
                "language-server".into(),
            ],
            env: clean_env(&[tc.php_dir()], state),
            cwd: project.dir.clone(),
            pull_diagnostics: false,
            settings: json!({}),
        },
        Language::Rust => ServerSpec {
            name: "rust-analyzer",
            program: tc
                .tools()
                .join("rust-analyzer")
                .join("bin")
                .join(exe("rust-analyzer")),
            args: vec![],
            // rust-analyzer may run the bundled rustc for target queries; nothing else.
            env: clean_env(&[tc.rust_sysroot().join("bin")], state),
            cwd: project.dir.clone(),
            // rust-analyzer serves its native diagnostics by pull only.
            pull_diagnostics: true,
            // Never cargo, never build scripts or proc-macros, never `cargo check`.
            // `sysrootQueryMetadata: none` stops it from running `cargo metadata` on the
            // sysroot (it tries even with a rust-project.json; the bundled toolchain has no
            // cargo anyway). Semantic diagnostics are "experimental" in rust-analyzer and
            // only analyse; without them it reports syntax errors only. E0282 ("type
            // annotations needed") is off: it fires on correct code such as
            // `let m: HashMap<i64, i64> = HashMap::new();`.
            settings: json!({
                "linkedProjects": [s(project.dir.join("rust-project.json"))],
                "cargo": {
                    "buildScripts": { "enable": false },
                    // Not `autoreload: false`: it stops the linked project from loading.
                    "sysrootQueryMetadata": "none"
                },
                "procMacro": { "enable": false },
                "checkOnSave": false,
                "files": { "watcher": "client" },
                "diagnostics": {
                    "enable": true,
                    "experimental": { "enable": true },
                    "disabled": ["E0282"]
                }
            }),
        },
    };
    Ok(spec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_tools_are_reported() {
        let tc = Toolchains::new(std::env::temp_dir().join("kb-bridge-nothing-here"));
        for lang in [Language::Python, Language::TypeScript, Language::Rust] {
            let why = unavailable_reason(lang, &tc).unwrap();
            assert!(why.starts_with("not installed"), "{why}");
        }
        let php = unavailable_reason(Language::Php, &tc).unwrap();
        if cfg!(windows) {
            assert!(php.contains("posix"), "{php}");
        }
    }

    #[test]
    fn the_environment_is_clean() {
        let state = std::env::temp_dir().join("kb-bridge-state");
        let env = clean_env(&[PathBuf::from("/runtime")], &state);
        let keys: Vec<&str> = env.iter().map(|(k, _)| k.as_str()).collect();
        for banned in [
            "HTTP_PROXY",
            "HTTPS_PROXY",
            "NODE_OPTIONS",
            "CARGO_HOME",
            "RUSTUP_HOME",
            "PYTHONPATH",
        ] {
            assert!(!keys.contains(&banned), "{banned}");
        }
        let get = |k: &str| {
            env.iter()
                .find(|(key, _)| key == k)
                .map(|(_, v)| v.clone())
                .unwrap()
        };
        assert_eq!(get("PATH"), "/runtime");
        assert!(get("HOME").starts_with(&state.display().to_string()));
        assert!(get("TEMP").starts_with(&state.display().to_string()));
    }
}
