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
        Language::Sql => "sql",
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
/// The parts of Node's API a program problem uses (ADR 0028), hand-written: not
/// @types/node. Only program problems' TypeScript/JavaScript projects get it, so a
/// function problem never sees `process` or `require`.
pub const NODE_SHIM_FILE: &str = "node-program.d.ts";
pub const NODE_SHIM: &str = r#"// KataBox: the Node APIs a program problem reads stdin and writes stdout with.
// Hand-written for the editor's checks (not @types/node); the judge runs real Node.
type KbEncoding = "utf8" | "utf-8" | "ascii" | "latin1";
interface Buffer extends Uint8Array {
  toString(encoding?: KbEncoding, start?: number, end?: number): string;
}
declare var Buffer: {
  from(data: string | ArrayLike<number>, encoding?: KbEncoding): Buffer;
  alloc(size: number): Buffer;
  concat(list: readonly Uint8Array[]): Buffer;
};
declare module "node:fs" {
  export function readFileSync(path: number | string, options: KbEncoding | { encoding: KbEncoding }): string;
  export function readFileSync(path: number | string): Buffer;
  export function readSync(fd: number, buffer: Uint8Array, offset?: number, length?: number, position?: number | null): number;
  export function writeSync(fd: number, data: string | Uint8Array): number;
}
declare module "fs" {
  export * from "node:fs";
}
interface KbReadStream {
  fd: number;
  setEncoding(encoding: KbEncoding): void;
  on(event: "data", listener: (chunk: string) => void): void;
  on(event: "end" | "close", listener: () => void): void;
}
interface KbWriteStream {
  write(chunk: string | Uint8Array): boolean;
}
declare var process: {
  stdin: KbReadStream;
  stdout: KbWriteStream;
  stderr: KbWriteStream;
  argv: string[];
  env: Record<string, string | undefined>;
  exit(code?: number): never;
};
declare function require(id: string): any;
"#;

pub fn prepare(
    projects_root: &Path,
    problem: &str,
    language: Language,
    text: &str,
    tc: &Toolchains,
) -> io::Result<Project> {
    prepare_with(projects_root, problem, language, text, false, tc)
}

/// [`prepare`], for a program problem when `program` is set: TypeScript and JavaScript
/// projects then also get [`NODE_SHIM`].
pub fn prepare_with(
    projects_root: &Path,
    problem: &str,
    language: Language,
    text: &str,
    program: bool,
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
    let shim = program && matches!(language, Language::TypeScript | Language::JavaScript);
    let shim_path = dir.join(NODE_SHIM_FILE);
    if shim {
        std::fs::write(&shim_path, NODE_SHIM)?;
    } else if shim_path.exists() {
        std::fs::remove_file(&shim_path)?;
    }
    let mut ts_files = vec![file_name.clone()];
    if shim {
        ts_files.push(NODE_SHIM_FILE.to_string());
    }

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
                "files": ts_files,
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
        Language::Sql => None,
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
    fn only_program_typescript_projects_get_the_node_shim() {
        let root = temp("shim");
        let tc = Toolchains::new(temp("tc-shim"));
        let read = |p: &Project| -> serde_json::Value {
            serde_json::from_str(&std::fs::read_to_string(p.dir.join("tsconfig.json")).unwrap())
                .unwrap()
        };
        // A program problem: the shim is written and listed with the solution.
        let p = prepare_with(&root, "tide-lows", Language::TypeScript, "code", true, &tc).unwrap();
        assert_eq!(
            std::fs::read_to_string(p.dir.join(NODE_SHIM_FILE)).unwrap(),
            NODE_SHIM
        );
        assert_eq!(
            read(&p)["files"],
            serde_json::json!(["solution.ts", NODE_SHIM_FILE])
        );
        let js = prepare_with(&root, "tide-lows", Language::JavaScript, "code", true, &tc).unwrap();
        assert!(js.dir.join(NODE_SHIM_FILE).is_file());
        // A function problem never sees it, even in a folder that once had it.
        let p = prepare_with(&root, "tide-lows", Language::TypeScript, "code", false, &tc).unwrap();
        assert!(!p.dir.join(NODE_SHIM_FILE).exists());
        assert_eq!(read(&p)["files"], serde_json::json!(["solution.ts"]));
        let p = prepare(&root, "two-sum", Language::TypeScript, "code", &tc).unwrap();
        assert!(!p.dir.join(NODE_SHIM_FILE).exists());
        // Other languages never get it.
        let py = prepare_with(&root, "tide-lows", Language::Python, "code", true, &tc).unwrap();
        assert!(!py.dir.join(NODE_SHIM_FILE).exists());
        // The shim is hand-written and small: no @types/node.
        assert!(NODE_SHIM.contains("declare module \"node:fs\"") && NODE_SHIM.len() < 4000);
    }

    #[test]
    fn sql_has_no_server_and_no_config() {
        let root = temp("sql");
        let tc = Toolchains::new(temp("tc-sql"));
        let p = prepare(&root, "late-returns", Language::Sql, "SELECT 1", &tc).unwrap();
        assert_eq!(p.language_id, "sql");
        assert_eq!(p.file.file_name().unwrap(), "solution.sql");
        assert_eq!(
            std::fs::read_dir(&p.dir).unwrap().count(),
            1,
            "only the solution"
        );
        assert_eq!(
            crate::spec::unavailable_reason(Language::Sql, &tc).as_deref(),
            Some("no language server for SQL")
        );
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
