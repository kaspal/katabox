//! The virtual project a language server works in (ADR 0014): one folder per problem and
//! language under the app's data folder, holding the solution file plus whatever the
//! server needs for correct types. The editor's text reaches the server through
//! didOpen/didChange; the file on disk is only written when a session starts.

use std::io;
use std::path::{Path, PathBuf};

use kb_core::Language;
use serde_json::json;
use url::Url;

use crate::spec::Toolchains;

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub dir: PathBuf,
    pub file: PathBuf,
    /// `file:///…/` with a trailing slash.
    pub root_uri: String,
    pub file_uri: String,
    /// LSP language id (`python`, `typescript`, `php`, `rust`).
    pub language_id: &'static str,
}

/// Problem ids are package folder names: lowercase letters, digits and `-`. Anything
/// else is refused so an id can never escape the projects folder.
pub fn valid_problem_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 100
        && id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !id.starts_with('-')
}

pub fn language_id(language: Language) -> &'static str {
    match language {
        Language::Python => "python",
        Language::JavaScript => "javascript",
        Language::TypeScript => "typescript",
        Language::Php => "php",
        Language::Rust => "rust",
    }
}

fn uri_of(path: &Path, dir: bool) -> io::Result<String> {
    let url = if dir {
        Url::from_directory_path(path)
    } else {
        Url::from_file_path(path)
    };
    url.map(String::from).map_err(|()| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("not absolute: {}", path.display()),
        )
    })
}

/// Create (or refresh) the project for `problem` in `language` under `projects_root`,
/// with `text` as the initial solution file.
pub fn prepare(
    projects_root: &Path,
    problem: &str,
    language: Language,
    text: &str,
    tc: &Toolchains,
) -> io::Result<Project> {
    if !valid_problem_id(problem) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("invalid problem id {problem:?}"),
        ));
    }
    let dir = projects_root.join(problem).join(language.id());
    std::fs::create_dir_all(&dir)?;
    let dir = std::path::absolute(&dir)?;
    let file_name = format!("solution.{}", language.extension());
    let file = dir.join(&file_name);
    std::fs::write(&file, text)?;

    let config: Option<(&str, serde_json::Value)> = match language {
        Language::Python => Some((
            "pyrightconfig.json",
            json!({
                "include": [file_name],
                "pythonVersion": "3.14",
                "typeCheckingMode": "standard",
                "reportMissingModuleSource": "none",
                // `node.next.next` on `ListNode | None` is natural list code: a warning,
                // not an error (ADR 0025, maintainer-approved).
                "reportOptionalMemberAccess": "warning",
            }),
        )),
        // Plain JavaScript (the editor's JS mode) is checked, but not strictly: no
        // implicit-any noise on untyped parameters. TypeScript stays strict.
        Language::TypeScript | Language::JavaScript => Some((
            "tsconfig.json",
            json!({
                "compilerOptions": {
                    "strict": language == Language::TypeScript,
                    "checkJs": language == Language::JavaScript,
                    "target": "es2024",
                    "lib": ["es2024"],
                    "module": "esnext",
                    "moduleResolution": "bundler",
                    "erasableSyntaxOnly": true,
                    "noEmit": true,
                    "types": [],
                    "allowJs": true,
                },
                "files": [file_name],
            }),
        )),
        Language::Php => Some((
            ".phpactor.json",
            json!({
                "indexer.enabled_watchers": [],
                "indexer.include_patterns": [format!("/{file_name}")],
                "language_server_phpstan.enabled": false,
                "language_server_psalm.enabled": false,
                "language_server_php_cs_fixer.enabled": false,
                "php_code_sniffer.enabled": false,
                "composer.enable": false,
                "behat.enabled": false,
                "prophecy.enabled": false,
                "symfony.enabled": false,
                "blackfire.enabled": false,
            }),
        )),
        Language::Rust => {
            let sysroot = tc.rust_sysroot();
            Some((
                "rust-project.json",
                json!({
                    "sysroot": sysroot,
                    // std's sources live outside the judge's sysroot (ADR 0015).
                    "sysroot_src": tc.rust_src_library(),
                    "crates": [{
                        "root_module": file,
                        "edition": "2024",
                        "deps": [],
                        "cfg": [],
                        "is_workspace_member": true,
                        "is_proc_macro": false,
                    }],
                }),
            ))
        }
    };
    if let Some((name, value)) = config {
        let body = serde_json::to_string_pretty(&value).map_err(io::Error::other)?;
        std::fs::write(dir.join(name), body)?;
    }
    Ok(Project {
        root_uri: uri_of(&dir, true)?,
        file_uri: uri_of(&file, false)?,
        dir,
        file,
        language_id: language_id(language),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("kb-bridge-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        d
    }

    #[test]
    fn problem_ids_cannot_escape() {
        for bad in [
            "", "..", "../x", "a/b", "a\\b", "C:", "Two-Sum", "-x", "x y",
        ] {
            assert!(!valid_problem_id(bad), "{bad}");
        }
        assert!(valid_problem_id("two-sum"));
        let tc = Toolchains::new(temp("tc"));
        let err = prepare(&temp("bad"), "../etc", Language::Python, "", &tc).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }

    #[test]
    fn writes_the_solution_and_config_per_language() {
        let root = temp("projects");
        let tc = Toolchains::new(temp("tc2"));
        let cases = [
            (Language::Python, "solution.py", "pyrightconfig.json"),
            (Language::TypeScript, "solution.ts", "tsconfig.json"),
            (Language::Php, "solution.php", ".phpactor.json"),
            (Language::Rust, "solution.rs", "rust-project.json"),
        ];
        for (lang, file, config) in cases {
            let p = prepare(&root, "two-sum", lang, "code", &tc).unwrap();
            assert_eq!(p.file.file_name().unwrap(), file);
            assert_eq!(std::fs::read_to_string(&p.file).unwrap(), "code");
            assert!(p.dir.join(config).is_file(), "{config}");
            assert!(p.root_uri.starts_with("file:///") && p.root_uri.ends_with('/'));
            assert!(
                p.file_uri.starts_with(&p.root_uri),
                "{} vs {}",
                p.file_uri,
                p.root_uri
            );
            assert!(p.dir.ends_with(Path::new("two-sum").join(lang.id())));
        }
        let ts: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("two-sum/typescript/tsconfig.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(ts["compilerOptions"]["erasableSyntaxOnly"], true);
        assert_eq!(ts["compilerOptions"]["strict"], true);
        // Optional member access on nodes is a warning (ADR 0025); the rest is standard.
        let py: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("two-sum/python/pyrightconfig.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(py["reportOptionalMemberAccess"], "warning");
        assert_eq!(py["typeCheckingMode"], "standard");
        // JavaScript mode: checked, not strict (ADR 0024).
        let p = prepare(&root, "two-sum", Language::JavaScript, "code", &tc).unwrap();
        assert_eq!(p.file.file_name().unwrap(), "solution.js");
        let js: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(p.dir.join("tsconfig.json")).unwrap())
                .unwrap();
        assert_eq!(js["compilerOptions"]["strict"], false);
        assert_eq!(js["compilerOptions"]["checkJs"], true);
        assert_eq!(js["files"][0], "solution.js");
        let rs: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(root.join("two-sum/rust/rust-project.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(rs["crates"][0]["is_proc_macro"], false);
        assert!(rs["crates"][0].get("build").is_none(), "no build scripts");
        assert!(rs["sysroot_src"].as_str().unwrap().ends_with("library"));
    }
}
