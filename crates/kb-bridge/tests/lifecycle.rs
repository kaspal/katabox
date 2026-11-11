//! Session lifecycle with a fake server (`kb-bridge-fake-lsp`): lazy start, relay through
//! the policy, clean environment, idle stop, crash → restart with backoff → unavailable.

use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use kb_bridge::spec::clean_env;
use kb_bridge::{BridgeError, LspEvent, LspManager, LspStatus, ServerSpec, SessionConfig, Sink};
use kb_core::Language;
use serde_json::{Value, json};

#[derive(Clone, Default)]
struct Events(Arc<Mutex<Vec<LspEvent>>>);

impl Events {
    fn sink(&self) -> Sink {
        let inner = self.0.clone();
        Arc::new(move |e| inner.lock().unwrap().push(e))
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
    fn count(&self, pred: impl Fn(&LspStatus) -> bool) -> usize {
        self.statuses().iter().filter(|s| pred(s)).count()
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
    async fn message(&self, pred: impl Fn(&Value) -> bool) -> Value {
        for _ in 0..400 {
            if let Some(m) = self.messages().into_iter().find(|m| pred(m)) {
                return m;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        panic!("message not received; got {:?}", self.messages());
    }
    /// Wait until at least `n` statuses match.
    async fn until(&self, n: usize, pred: impl Fn(&LspStatus) -> bool) {
        for _ in 0..400 {
            if self.count(&pred) >= n {
                return;
            }
            tokio::time::sleep(Duration::from_millis(25)).await;
        }
        panic!("status not reached; got {:?}", self.statuses());
    }
}

fn temp(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("kb-bridge-life-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&d);
    std::fs::create_dir_all(&d).unwrap();
    d
}

fn manager(name: &str, args: &'static [&'static str], config: SessionConfig) -> LspManager {
    let dir = temp(name);
    LspManager::new(dir.join("root"), &dir.join("data"))
        .with_config(config)
        .with_spec(Arc::new(move |_lang, project, state: &Path| {
            std::fs::create_dir_all(state.join("tmp")).unwrap();
            Ok(ServerSpec {
                name: "fake",
                program: PathBuf::from(env!("CARGO_BIN_EXE_kb-bridge-fake-lsp")),
                args: args.iter().map(|s| s.to_string()).collect(),
                env: clean_env(&[], state),
                cwd: project.dir.clone(),
                settings: json!({ "fake": { "safe": true } }),
                pull_diagnostics: false,
            })
        }))
}

fn fast() -> SessionConfig {
    SessionConfig {
        idle: Duration::from_secs(30),
        backoff_base: Duration::from_millis(50),
        backoff_max: Duration::from_millis(200),
        max_crashes: 3,
        crash_window: Duration::from_secs(60),
        stop_grace: Duration::from_secs(2),
    }
}

fn req(id: i64, method: &str, params: Value) -> String {
    json!({ "jsonrpc": "2.0", "id": id, "method": method, "params": params }).to_string()
}

fn note(method: &str, params: Value) -> String {
    json!({ "jsonrpc": "2.0", "method": method, "params": params }).to_string()
}

const ALLOWED_ENV: &[&str] = &[
    "SystemRoot",
    "windir",
    "SystemDrive",
    "PATH",
    "TEMP",
    "TMP",
    "TMPDIR",
    "HOME",
    "USERPROFILE",
    "APPDATA",
    "LOCALAPPDATA",
    "XDG_CONFIG_HOME",
    "XDG_CACHE_HOME",
    "XDG_DATA_HOME",
];

#[tokio::test(flavor = "multi_thread")]
async fn starts_lazily_and_relays_through_the_policy() {
    let m = manager("relay", &[], fast());
    // Nothing runs before the first connect.
    assert!(matches!(
        m.send(Language::Python, "{}".into()).await,
        Err(BridgeError::NotConnected(Language::Python))
    ));

    let ev = Events::default();
    let conn = m
        .connect("two-sum", Language::Python, "x = 1\n", ev.sink())
        .await
        .unwrap();
    assert!(conn.file_uri.starts_with(&conn.root_uri) && conn.file_uri.ends_with("solution.py"));
    assert_eq!(conn.language_id, "python");

    // The UI's initialize asks for the whole disk and its own options; the server sees
    // only the project root and the bridge's settings.
    m.send(
        Language::Python,
        req(
            1,
            "initialize",
            json!({
                "rootUri": "file:///C:/",
                "workspaceFolders": [{ "uri": "file:///C:/", "name": "all" }],
                "initializationOptions": { "evil": true },
                "capabilities": {}
            }),
        ),
    )
    .await
    .unwrap();
    ev.until(1, |s| *s == LspStatus::Ready).await;
    m.send(Language::Python, note("textDocument/didOpen", json!({
        "textDocument": { "uri": conn.file_uri, "languageId": "python", "version": 1, "text": "x" }
    })))
    .await
    .unwrap();

    let diag = ev
        .message(|m| m["method"] == "textDocument/publishDiagnostics")
        .await;
    let seen = &diag["params"]["diagnostics"][0]["data"];
    assert_eq!(seen["rootUri"], conn.root_uri);
    assert_eq!(
        seen["workspaceFolders"],
        json!([{ "uri": conn.root_uri, "name": "katabox" }])
    );
    assert_eq!(
        seen["initializationOptions"],
        json!({ "fake": { "safe": true } })
    );
    // A clean environment: nothing inherited beyond the allow-list.
    for k in seen["env"].as_array().unwrap() {
        let k = k.as_str().unwrap();
        assert!(
            ALLOWED_ENV.iter().any(|a| a.eq_ignore_ascii_case(k)),
            "unexpected variable {k}"
        );
    }
    assert!(
        seen["cwd"].as_str().unwrap().ends_with("python"),
        "{}",
        seen["cwd"]
    );

    // The server's configuration request is answered by the bridge, never by the UI.
    let configured = ev.message(|m| m["method"] == "fake/configured").await;
    assert_eq!(configured["params"], json!([{ "safe": true }]));
    assert!(
        !ev.messages()
            .iter()
            .any(|m| m["method"] == "workspace/configuration")
    );

    // Blocked: commands, and documents outside the project.
    m.send(
        Language::Python,
        req(2, "workspace/executeCommand", json!({ "command": "x" })),
    )
    .await
    .unwrap();
    assert_eq!(ev.message(|m| m["id"] == 2).await["error"]["code"], -32601);
    m.send(Language::Python, req(3, "textDocument/hover", json!({
        "textDocument": { "uri": "file:///C:/Windows/win.ini" }, "position": { "line": 0, "character": 0 }
    })))
    .await
    .unwrap();
    assert_eq!(ev.message(|m| m["id"] == 3).await["error"]["code"], -32601);

    m.disconnect(Language::Python).await;
    assert_eq!(ev.statuses().first(), Some(&LspStatus::Starting));
    assert_eq!(ev.statuses().last(), Some(&LspStatus::Stopped));
    assert!(m.send(Language::Python, "{}".into()).await.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn stops_when_idle_and_starts_again_on_connect() {
    let mut config = fast();
    config.idle = Duration::from_millis(400);
    let m = manager("idle", &[], config);
    let ev = Events::default();
    m.connect("two-sum", Language::Rust, "", ev.sink())
        .await
        .unwrap();
    m.send(
        Language::Rust,
        req(1, "initialize", json!({ "capabilities": {} })),
    )
    .await
    .unwrap();
    ev.until(1, |s| *s == LspStatus::Ready).await;
    ev.until(1, |s| *s == LspStatus::Stopped).await;
    assert!(m.send(Language::Rust, "{}".into()).await.is_err());

    let ev2 = Events::default();
    m.connect("two-sum", Language::Rust, "", ev2.sink())
        .await
        .unwrap();
    ev2.until(1, |s| *s == LspStatus::Starting).await;
    m.shutdown().await;
    ev2.until(1, |s| *s == LspStatus::Stopped).await;
}

#[tokio::test(flavor = "multi_thread")]
async fn restarts_after_crashes_with_backoff_then_gives_up() {
    let m = manager("crash", &[], fast());
    let ev = Events::default();
    let conn = m
        .connect("two-sum", Language::TypeScript, "", ev.sink())
        .await
        .unwrap();
    for round in 1..=3 {
        ev.until(round, |s| *s == LspStatus::Starting).await;
        // The fake server dies on this edit, as a buggy server would.
        m.send(Language::TypeScript, note("textDocument/didChange", json!({
            "textDocument": { "uri": format!("{}solution.ts", conn.root_uri), "version": 2 },
            "contentChanges": [{ "text": "crash" }]
        })))
        .await
        .unwrap();
        if round < 3 {
            ev.until(round, |s| matches!(s, LspStatus::Restarting { .. }))
                .await;
        }
    }
    ev.until(
        1,
        |s| matches!(s, LspStatus::Unavailable { reason } if reason.contains("keeps crashing")),
    )
    .await;
    let delays: Vec<u64> = ev
        .statuses()
        .iter()
        .filter_map(|s| match s {
            LspStatus::Restarting { delay_ms, .. } => Some(*delay_ms),
            _ => None,
        })
        .collect();
    assert_eq!(delays, [50, 100], "exponential backoff");
    assert!(m.send(Language::TypeScript, "{}".into()).await.is_err());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_server_that_cannot_start_reports_its_stderr() {
    let m = manager("die", &["--die-at-start"], fast());
    let ev = Events::default();
    m.connect("two-sum", Language::Python, "", ev.sink())
        .await
        .unwrap();
    ev.until(1, |s| matches!(s, LspStatus::Unavailable { reason } if reason.contains("told to die at start")))
        .await;
}

#[tokio::test(flavor = "multi_thread")]
async fn bad_problem_ids_and_missing_tools_are_errors() {
    let dir = temp("missing");
    let m = LspManager::new(dir.join("empty-root"), &dir);
    let ev = Events::default();
    assert!(matches!(
        m.connect("../x", Language::Python, "", ev.sink()).await,
        Err(BridgeError::InvalidProblem(_))
    ));
    let err = m
        .connect("two-sum", Language::Python, "", ev.sink())
        .await
        .unwrap_err();
    assert!(
        matches!(err, BridgeError::Unavailable(ref r) if r.starts_with("not installed")),
        "{err}"
    );
    assert!(matches!(
        ev.statuses().last(),
        Some(LspStatus::Unavailable { .. })
    ));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_client_exit_is_a_stop_not_a_crash() {
    let m = manager("exit", &[], fast());
    let ev = Events::default();
    m.connect("two-sum", Language::Python, "", ev.sink())
        .await
        .unwrap();
    m.send(Language::Python, req(1, "shutdown", Value::Null))
        .await
        .unwrap();
    m.send(Language::Python, note("exit", Value::Null))
        .await
        .unwrap();
    ev.until(1, |s| *s == LspStatus::Stopped).await;
    assert_eq!(ev.count(|s| matches!(s, LspStatus::Restarting { .. })), 0);
}
