//! The real language servers through the bridge, on two-sum: start, open the solution,
//! get a completion and a diagnostic. Needs the dev tools:
//!
//! ```text
//! pwsh scripts/fetch-toolchains.ps1      # toolchains + language servers in target/katabox
//! cargo test -p kb-bridge --test servers_windows
//! ```
#![cfg(windows)]

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kb_bridge::{LspEvent, LspManager, LspStatus, Sink};
use kb_core::Language;
use serde_json::{Value, json};

fn root() -> PathBuf {
    let r = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/katabox");
    std::path::absolute(r).unwrap()
}

#[derive(Clone, Default)]
struct Events(Arc<Mutex<Vec<LspEvent>>>);

impl Events {
    fn sink(&self) -> Sink {
        let inner = self.0.clone();
        Arc::new(move |e| inner.lock().unwrap().push(e))
    }
    fn messages(&self) -> Vec<Value> {
        let all = self.0.lock().unwrap();
        all.iter()
            .filter_map(|e| match e {
                LspEvent::Message { message } => serde_json::from_str(message).ok(),
                LspEvent::Status { .. } => None,
            })
            .collect()
    }
    fn statuses(&self) -> Vec<LspStatus> {
        let all = self.0.lock().unwrap();
        all.iter()
            .filter_map(|e| match e {
                LspEvent::Status { status } => Some(status.clone()),
                LspEvent::Message { .. } => None,
            })
            .collect()
    }
    async fn message(&self, secs: u64, pred: impl Fn(&Value) -> bool) -> Option<Value> {
        for _ in 0..secs * 10 {
            if let Some(m) = self.messages().into_iter().find(|m| pred(m)) {
                return Some(m);
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        None
    }
}

fn items(result: &Value) -> Vec<String> {
    let list = result.get("items").unwrap_or(result);
    list.as_array()
        .map(|a| {
            a.iter()
                .filter_map(|i| i["label"].as_str().map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Open `code` and ask for completion at (line, col) until the server answers with items
/// (servers may need a few seconds to load their library), then wait for a diagnostic.
/// Returns the (severity, message) of that notification's errors (1) and warnings (2).
async fn check(
    language: Language,
    code: &str,
    line: u32,
    col: u32,
    expect_item: &str,
    expect_diag: &str,
    // Lines (by a fragment) that may have diagnostics; any other line must have none.
    broken_lines: &[&str],
) -> Vec<(u64, String)> {
    static RUN: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
    let data = root().join("..").join(format!(
        "kb-bridge-real-{}-{}-{}",
        language.id(),
        std::process::id(),
        RUN.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&data);
    let m = LspManager::new(root(), &data);
    assert_eq!(
        m.unavailable_reason(language),
        None,
        "run scripts/fetch-toolchains.ps1"
    );
    let ev = Events::default();
    let conn = m
        .connect("two-sum", language, code, ev.sink())
        .await
        .unwrap();
    let send = |v: Value| {
        let m = &m;
        async move { m.send(language, conn.session, v.to_string()).await.unwrap() }
    };
    send(
        json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "processId": null, "rootUri": null,
        "capabilities": {
            "textDocument": {
                "completion": { "completionItem": { "snippetSupport": false } },
                "hover": { "contentFormat": ["markdown", "plaintext"] },
                "publishDiagnostics": {}
            },
            "general": { "positionEncodings": ["utf-16"] }
        }
    } }),
    )
    .await;
    let init = ev
        .message(60, |m| m["id"] == 1)
        .await
        .expect("initialize answered");
    assert!(init.get("error").is_none(), "{init}");
    send(json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} })).await;
    send(json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": { "uri": conn.file_uri, "languageId": conn.language_id, "version": 1, "text": code }
    } }))
    .await;

    // A cold rust-analyzer indexes std first; under a loaded machine (the whole test suite)
    // that has taken over 20 s. Asking again every 0.5 s returns as soon as it is ready.
    let mut labels = Vec::new();
    for attempt in 0..180 {
        let id = 100 + attempt;
        send(
            json!({ "jsonrpc": "2.0", "id": id, "method": "textDocument/completion", "params": {
            "textDocument": { "uri": conn.file_uri }, "position": { "line": line, "character": col }
        } }),
        )
        .await;
        if let Some(r) = ev.message(10, |m| m["id"] == id).await {
            labels = items(&r["result"]);
            if labels.iter().any(|l| l.contains(expect_item)) {
                break;
            }
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    if !labels.iter().any(|l| l.contains(expect_item)) {
        let logs: Vec<String> = ev
            .messages()
            .iter()
            .filter(|m| {
                m["method"]
                    .as_str()
                    .is_some_and(|x| x.starts_with("window/") || x.starts_with("$/"))
            })
            .map(|m| m["params"].to_string().chars().take(300).collect())
            .collect();
        panic!(
            "{language}: no {expect_item:?} in {labels:?}
server said: {logs:#?}"
        );
    }

    // Diagnostics arrive without an edit (rust-analyzer: the bridge pulls again when the
    // server asks for a refresh after loading its project).
    let diag = ev
        .message(60, |m| {
            m["method"] == "textDocument/publishDiagnostics"
                && m["params"]["diagnostics"].as_array().is_some_and(|d| {
                    d.iter().any(|x| {
                        x["message"]
                            .as_str()
                            .unwrap_or_default()
                            .contains(expect_diag)
                    })
                })
        })
        .await;
    if diag.is_none() {
        let got: Vec<String> = ev
            .messages()
            .iter()
            .filter(|m| m["method"] == "textDocument/publishDiagnostics")
            .flat_map(|m| {
                m["params"]["diagnostics"]
                    .as_array()
                    .cloned()
                    .unwrap_or_default()
            })
            .map(|d| d["message"].as_str().unwrap_or_default().to_string())
            .collect();
        let methods: Vec<String> = ev
            .messages()
            .iter()
            .filter_map(|m| m["method"].as_str().map(str::to_owned))
            .collect();
        panic!(
            "{language}: no diagnostic containing {expect_diag:?}; got {got:?}; statuses {:?}; server methods {methods:?}",
            ev.statuses()
        );
    }
    // Correct code gets no errors or warnings (hints such as "never read" are fine): only
    // the deliberately broken lines do.
    let mut found = Vec::new();
    for d in diag.as_ref().unwrap()["params"]["diagnostics"]
        .as_array()
        .unwrap()
    {
        if d["severity"].as_u64().unwrap_or(1) > 2 {
            continue;
        }
        found.push((
            d["severity"].as_u64().unwrap_or(1),
            d["message"].as_str().unwrap_or_default().to_string(),
        ));
        let line = d["range"]["start"]["line"].as_u64().unwrap() as usize;
        let text = code.lines().nth(line).unwrap_or_default();
        assert!(
            broken_lines.iter().any(|b| text.contains(b)),
            "{language}: diagnostic on correct line {line} ({text:?}): {}",
            d["message"]
        );
    }
    // The editor matches diagnostics to its file by exact URI, so the bridge must hand
    // back the URI it gave out, however the server spells it.
    assert_eq!(
        diag.unwrap()["params"]["uri"].as_str(),
        Some(conn.file_uri.as_str()),
        "{language}: diagnostics URI"
    );
    assert!(ev.statuses().contains(&LspStatus::Ready));
    m.shutdown().await;
    assert_eq!(ev.statuses().last(), Some(&LspStatus::Stopped));
    found
}

#[tokio::test(flavor = "multi_thread")]
async fn basedpyright_completes_and_diagnoses() {
    let code = "def two_sum(nums: list[int], target: int) -> list[int]:\n    seen: dict[int, int] = {}\n    seen.ite\n    return undefined_name\n";
    check(
        Language::Python,
        code,
        2,
        12,
        "items",
        "undefined_name",
        &["seen.ite", "undefined_name"],
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn typescript_language_server_completes_and_diagnoses() {
    let code = "export function twoSum(nums: number[], target: number): number[] {\n  const seen = new Map<number, number>();\n  seen.se\n  return undefinedName;\n}\n";
    check(
        Language::TypeScript,
        code,
        2,
        9,
        "set",
        "undefinedName",
        &["seen.se", "undefinedName"],
    )
    .await;
}

/// The editor's JS mode (ADR 0024): plain JavaScript is checked without implicit-any
/// noise, while the same code as TypeScript gets it (strict). Real errors still show.
#[tokio::test(flavor = "multi_thread")]
async fn javascript_mode_is_checked_without_implicit_any() {
    let code = "export function twoSum(nums, target) {
  const seen = new Map();
  seen.se
  return undefinedName;
}
";
    // JS: only the deliberately broken lines (check() fails on any other diagnostic).
    check(
        Language::JavaScript,
        code,
        2,
        9,
        "set",
        "undefinedName",
        &["seen.se", "undefinedName"],
    )
    .await;
    let strict = check(
        Language::TypeScript,
        code,
        2,
        9,
        "set",
        "undefinedName",
        &["seen.se", "undefinedName", "twoSum(nums, target)"],
    )
    .await;
    assert!(
        strict
            .iter()
            .any(|(_, m)| m.contains("implicitly has an 'any' type")),
        "{strict:?}"
    );
}

/// Linked-list code (ADR 0025): `head.next.next` on `ListNode | None` is a warning
/// (reportOptionalMemberAccess, ADR 0014's settings), not an error; real errors stay
/// errors.
#[tokio::test(flavor = "multi_thread")]
async fn basedpyright_warns_on_optional_node_access() {
    let code = "class ListNode:
    def __init__(self, val: int = 0, next: ListNode | None = None):
        self.val = val
        self.next = next


def third(head: ListNode | None) -> int:
    seen: list[int] = []
    seen.app
    value = head.next.next.val
    return value + undefined_name
";
    let found = check(
        Language::Python,
        code,
        8,
        12,
        "append",
        "undefined_name",
        &["seen.app", "head.next.next", "undefined_name"],
    )
    .await;
    let optional: Vec<&(u64, String)> = found
        .iter()
        .filter(|(_, m)| m.contains("\"next\"") || m.contains("\"val\""))
        .collect();
    assert!(!optional.is_empty(), "{found:?}");
    assert!(optional.iter().all(|(sev, _)| *sev == 2), "{found:?}");
    assert!(
        found
            .iter()
            .any(|(sev, m)| *sev == 1 && m.contains("undefined_name")),
        "{found:?}"
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn rust_analyzer_completes_and_diagnoses_without_cargo() {
    // Well-formed around the cursor: rust-analyzer completes `seen.ins|()`.
    let code = "use std::collections::HashMap;\n\npub fn two_sum(nums: Vec<i64>, target: i64) -> Vec<i64> {\n    let mut seen: HashMap<i64, i64> = HashMap::new();\n    seen.ins();\n    vec![]\n}\n\nfn broken(seen: HashMap<i64, i64>) { seen.no_such_method(); }\n";
    // An unknown method is a stable native diagnostic (no cargo check involved).
    check(
        Language::Rust,
        code,
        4,
        12,
        "insert",
        "no_such_method",
        &["seen.ins", "no_such_method"],
    )
    .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn php_is_unavailable_on_windows_with_the_reason() {
    let m = LspManager::new(root(), &std::env::temp_dir().join("kb-bridge-real-php"));
    let reason = m.unavailable_reason(Language::Php).unwrap();
    assert!(reason.contains("posix"), "{reason}");
}

/// Open `code` as `problem`'s solution (`program`: the Node typings shim) and return the
/// error-or-warning diagnostics (severity, message, line) once the server has published
/// some containing `wait_for`.
async fn diagnostics_of(
    m: &LspManager,
    problem: &str,
    code: &str,
    program: bool,
    wait_for: &str,
) -> Vec<(u64, String, u64)> {
    let ev = Events::default();
    let conn = m
        .connect_with(problem, Language::TypeScript, code, program, ev.sink())
        .await
        .unwrap();
    let send = |v: Value| async move {
        m.send(Language::TypeScript, conn.session, v.to_string())
            .await
            .unwrap()
    };
    send(
        json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
        "processId": null, "rootUri": null,
        "capabilities": { "textDocument": { "publishDiagnostics": {} } }
    } }),
    )
    .await;
    assert!(
        ev.message(60, |x| x["id"] == 1).await.is_some(),
        "{problem}: initialize"
    );
    send(json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} })).await;
    send(json!({ "jsonrpc": "2.0", "method": "textDocument/didOpen", "params": {
        "textDocument": { "uri": conn.file_uri, "languageId": conn.language_id, "version": 1, "text": code }
    } }))
    .await;
    let diag = ev
        .message(90, |x| {
            x["method"] == "textDocument/publishDiagnostics"
                && x["params"]["diagnostics"].as_array().is_some_and(|d| {
                    d.iter()
                        .any(|y| y["message"].as_str().unwrap_or_default().contains(wait_for))
                })
        })
        .await
        .unwrap_or_else(|| panic!("{problem}: no diagnostics mentioning {wait_for:?}"));
    diag["params"]["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|d| d["severity"].as_u64().unwrap_or(1) <= 2)
        .map(|d| {
            (
                d["severity"].as_u64().unwrap_or(1),
                d["message"].as_str().unwrap_or_default().to_string(),
                d["range"]["start"]["line"].as_u64().unwrap_or(0),
            )
        })
        .collect()
}

/// Program problems in TypeScript (ADR 0028) see Node's stdin/stdout APIs through the
/// hand-written shim: every catalogue reference has no error but a deliberate probe on
/// its last line. A function problem's project has no shim: the same code is flagged.
#[tokio::test(flavor = "multi_thread")]
async fn typescript_program_references_have_no_false_errors() {
    let data = root()
        .join("..")
        .join(format!("kb-bridge-program-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&data);
    let m = LspManager::new(root(), &data);
    assert_eq!(
        m.unavailable_reason(Language::TypeScript),
        None,
        "run scripts/fetch-toolchains.ps1"
    );
    let problems = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../content/problems");
    let probe = "\nconst kbProbe: number = \"not a number\";\n";
    let mut checked = 0;
    let mut dirs: Vec<_> = std::fs::read_dir(&problems)
        .unwrap()
        .flatten()
        .map(|e| e.path())
        .collect();
    dirs.sort();
    for dir in dirs {
        let toml = std::fs::read_to_string(dir.join("problem.toml")).unwrap_or_default();
        if !toml.contains("mode = \"program\"") {
            continue;
        }
        let id = dir.file_name().unwrap().to_string_lossy().into_owned();
        let code = std::fs::read_to_string(dir.join("solutions/ref.ts")).unwrap() + probe;
        let probe_line = code.lines().count() as u64 - 1;
        let found = diagnostics_of(&m, &id, &code, true, "not assignable").await;
        let others: Vec<_> = found
            .iter()
            .filter(|(_, _, line)| *line != probe_line)
            .collect();
        assert!(
            others.is_empty(),
            "{id}: false errors with the shim: {others:?}"
        );
        assert_eq!(
            found.len(),
            1,
            "{id}: the probe is the only diagnostic: {found:?}"
        );
        checked += 1;
    }
    assert!(checked >= 35, "{checked} program problems");
    // Function mode: no shim, so `process` and `node:fs` are unknown.
    let code =
        std::fs::read_to_string(problems.join("tide-lows/solutions/ref.ts")).unwrap() + probe;
    let found = diagnostics_of(&m, "tide-lows", &code, false, "not assignable").await;
    assert!(
        found
            .iter()
            .any(|(_, msg, _)| msg.contains("process") || msg.contains("node:fs")),
        "function mode must not see the shim: {found:?}"
    );
    m.shutdown().await;
}
