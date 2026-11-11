//! One language server session: the process, relaying through the policy, idle stop,
//! and crash restarts with backoff (ADR 0014). Runs as one tokio task; the UI talks to it
//! through a [`Session`] handle.

use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;

use serde::Serialize;
use serde_json::{Value, json};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::{mpsc, oneshot};
use tokio::time::{Instant, sleep_until, timeout};

use crate::framing::{read_message, write_message};
use crate::policy::{FromClient, FromServer, Policy};
use crate::process::{ServerProcess, spawn};
use crate::project::Project;
use crate::spec::ServerSpec;

/// What the UI shows ("Python language server: ready").
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(tag = "state", rename_all = "snake_case")]
pub enum LspStatus {
    /// The process is starting; the UI should (re)initialize its client.
    Starting,
    /// The server answered `initialize`.
    Ready,
    /// The server crashed; it restarts after `delay_ms`.
    Restarting { attempt: u32, delay_ms: u64 },
    /// No server for now (not installed, not supported here, or crashing repeatedly).
    Unavailable { reason: String },
    /// Stopped after being idle, or on request. The next connect starts it again.
    Stopped,
}

/// Sent to the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LspEvent {
    /// A JSON-RPC message from the server (already through the policy).
    Message {
        message: String,
    },
    Status {
        status: LspStatus,
    },
}

pub type Sink = Arc<dyn Fn(LspEvent) + Send + Sync>;

#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Stop after this long without a message from the UI.
    pub idle: Duration,
    pub backoff_base: Duration,
    pub backoff_max: Duration,
    /// This many crashes within `crash_window` makes the server unavailable.
    pub max_crashes: usize,
    pub crash_window: Duration,
    /// Time allowed for shutdown/exit before the process tree is killed.
    pub stop_grace: Duration,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            idle: Duration::from_secs(5 * 60),
            backoff_base: Duration::from_secs(1),
            backoff_max: Duration::from_secs(30),
            max_crashes: 5,
            crash_window: Duration::from_secs(10 * 60),
            stop_grace: Duration::from_secs(2),
        }
    }
}

enum Cmd {
    Send(String),
    SetSink(Sink),
    Stop(oneshot::Sender<()>),
}

/// Handle to a running session task.
pub struct Session {
    tx: mpsc::UnboundedSender<Cmd>,
    alive: Arc<AtomicBool>,
    pub problem: String,
    pub project: Project,
}

impl Session {
    pub fn start(
        problem: String,
        project: Project,
        spec: ServerSpec,
        sink: Sink,
        config: SessionConfig,
    ) -> Session {
        let (tx, rx) = mpsc::unbounded_channel();
        let alive = Arc::new(AtomicBool::new(true));
        let policy = Policy {
            root_uri: project.root_uri.clone(),
            settings: spec.settings.clone(),
        };
        let flag = alive.clone();
        tokio::spawn(async move {
            run(spec, policy, sink, config, rx).await;
            flag.store(false, Ordering::SeqCst);
        });
        Session {
            tx,
            alive,
            problem,
            project,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst) && !self.tx.is_closed()
    }

    /// Queue a message from the UI. False when the session has ended.
    pub fn send(&self, message: String) -> bool {
        self.is_alive() && self.tx.send(Cmd::Send(message)).is_ok()
    }

    pub fn set_sink(&self, sink: Sink) {
        let _ = self.tx.send(Cmd::SetSink(sink));
    }

    /// Stop gracefully and wait for it.
    pub async fn stop(&self) {
        let (done, wait) = oneshot::channel();
        if self.tx.send(Cmd::Stop(done)).is_ok() {
            let _ = wait.await;
        }
    }
}

fn status(sink: &Sink, s: LspStatus) {
    sink(LspEvent::Status { status: s });
}

async fn write(proc: &mut ServerProcess, v: &Value) {
    // A broken pipe means the server is dying; its exit is handled by the main loop.
    let _ = write_message(&mut proc.stdin, v.to_string().as_bytes()).await;
}

async fn stop_process(mut proc: ServerProcess, grace: Duration) {
    write(
        &mut proc,
        &json!({ "jsonrpc": "2.0", "id": "kb-shutdown", "method": "shutdown" }),
    )
    .await;
    write(&mut proc, &json!({ "jsonrpc": "2.0", "method": "exit" })).await;
    if timeout(grace, &mut proc.exited).await.is_err() {
        proc.kill();
        let _ = timeout(grace, &mut proc.exited).await;
    }
    // Dropping `proc` closes the job: anything the server started dies too.
}

/// Debounce before pulling diagnostics after an edit.
const PULL_DELAY: Duration = Duration::from_millis(300);

enum End {
    Crashed(String),
    /// The server exited because the UI sent `exit`.
    Exited,
    Stopped(Option<oneshot::Sender<()>>),
    Closed,
}

async fn run(
    spec: ServerSpec,
    policy: Policy,
    mut sink: Sink,
    config: SessionConfig,
    mut rx: mpsc::UnboundedReceiver<Cmd>,
) {
    let mut crashes: VecDeque<Instant> = VecDeque::new();
    loop {
        status(&sink, LspStatus::Starting);
        let mut proc = match spawn(&spec) {
            Ok(p) => p,
            Err(e) => {
                status(
                    &sink,
                    LspStatus::Unavailable {
                        reason: format!("{} could not start: {e}", spec.name),
                    },
                );
                return;
            }
        };

        // Server → bridge: framed messages, parsed.
        let (srv_tx, mut srv_rx) = mpsc::unbounded_channel::<Value>();
        let mut tasks = Vec::new();
        if let Some(stdout) = proc.stdout.take() {
            tasks.push(tokio::spawn(async move {
                let mut r = BufReader::new(stdout);
                while let Ok(Some(body)) = read_message(&mut r).await {
                    if let Ok(v) = serde_json::from_slice::<Value>(&body) {
                        if srv_tx.send(v).is_err() {
                            break;
                        }
                    }
                }
            }));
        }
        // Keep the last stderr lines to explain a crash.
        let last_err: Arc<Mutex<VecDeque<String>>> = Arc::default();
        if let Some(stderr) = proc.stderr.take() {
            let buf = last_err.clone();
            tasks.push(tokio::spawn(async move {
                let mut lines = BufReader::new(stderr).lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    if line.trim().is_empty() {
                        continue;
                    }
                    let mut b = buf.lock().unwrap_or_else(|e| e.into_inner());
                    b.push_back(line);
                    if b.len() > 20 {
                        b.pop_front();
                    }
                }
            }));
        }

        let mut init_id: Option<Value> = None;
        let mut ready = false;
        // The UI asked the server to exit: its exit is a stop, not a crash.
        let mut exit_requested = false;
        let mut idle_at = Instant::now() + config.idle;
        // Pull diagnostics (servers with `pull_diagnostics`): the document to pull and
        // when, and the requests in flight by id.
        let mut pull: Option<(Instant, String)> = None;
        let mut last_doc: Option<String> = None;
        let mut pulls: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        let mut pull_seq: u64 = 0;
        let end = loop {
            let pull_at = pull.as_ref().map(|(t, _)| *t);
            tokio::select! {
                _ = sleep_until(pull_at.unwrap_or(idle_at)), if pull_at.is_some() => {
                    if let Some((_, uri)) = pull.take() {
                        pull_seq += 1;
                        let id = format!("kb-diag-{pull_seq}");
                        pulls.insert(id.clone(), uri.clone());
                        write(&mut proc, &json!({
                            "jsonrpc": "2.0", "id": id, "method": "textDocument/diagnostic",
                            "params": { "textDocument": { "uri": uri } }
                        })).await;
                    }
                }
                cmd = rx.recv() => match cmd {
                    None => break End::Closed,
                    Some(Cmd::Stop(done)) => break End::Stopped(Some(done)),
                    Some(Cmd::SetSink(s)) => sink = s,
                    Some(Cmd::Send(text)) => {
                        idle_at = Instant::now() + config.idle;
                        let Ok(v) = serde_json::from_str::<Value>(&text) else { continue };
                        match policy.from_client(v) {
                            FromClient::Forward(v) => {
                                match v.get("method").and_then(Value::as_str) {
                                    Some("initialize") => init_id = v.get("id").cloned(),
                                    Some("exit") => exit_requested = true,
                                    Some("textDocument/didOpen" | "textDocument/didChange") if spec.pull_diagnostics => {
                                        if let Some(uri) = v.pointer("/params/textDocument/uri").and_then(Value::as_str) {
                                            last_doc = Some(uri.to_string());
                                            pull = Some((Instant::now() + PULL_DELAY, uri.to_string()));
                                        }
                                    }
                                    _ => {}
                                }
                                write(&mut proc, &v).await;
                            }
                            FromClient::Reply(v) => sink(LspEvent::Message { message: v.to_string() }),
                            FromClient::Drop => {}
                        }
                    }
                },
                msg = srv_rx.recv() => match msg {
                    // Our own diagnostic pulls: hand the result to the UI as a push.
                    Some(v) if v.get("method").is_none()
                        && v.get("id").and_then(Value::as_str).is_some_and(|id| pulls.contains_key(id)) =>
                    {
                        let id = v["id"].as_str().unwrap_or_default().to_string();
                        if let (Some(uri), Some(items)) = (pulls.remove(&id), v.pointer("/result/items")) {
                            sink(LspEvent::Message { message: json!({
                                "jsonrpc": "2.0", "method": "textDocument/publishDiagnostics",
                                "params": { "uri": uri, "diagnostics": items }
                            }).to_string() });
                        }
                    }
                    Some(v) => {
                        // The server says diagnostics changed (e.g. its project finished loading).
                        if spec.pull_diagnostics && v.get("method").and_then(Value::as_str) == Some("workspace/diagnostic/refresh") {
                            if let Some(uri) = &last_doc {
                                pull = Some((Instant::now() + PULL_DELAY, uri.clone()));
                            }
                        }
                        match policy.from_server(v) {
                        FromServer::Forward(v) => {
                            let answers_init = !ready
                                && init_id.is_some()
                                && v.get("method").is_none()
                                && v.get("id") == init_id.as_ref();
                            sink(LspEvent::Message { message: v.to_string() });
                            if answers_init {
                                ready = true;
                                status(&sink, LspStatus::Ready);
                            }
                        }
                        FromServer::Reply(v) => write(&mut proc, &v).await,
                        FromServer::Drop => {}
                        }
                    }
                    None => {
                        // Output closed: the process is ending.
                        let code = (&mut proc.exited).await.ok().flatten();
                        break if exit_requested { End::Exited } else { End::Crashed(exit_text(code)) };
                    }
                },
                exit = &mut proc.exited => {
                    let code = exit.ok().flatten();
                    break if exit_requested { End::Exited } else { End::Crashed(exit_text(code)) };
                }
                _ = sleep_until(idle_at) => break End::Stopped(None),
            }
        };

        for t in &tasks {
            t.abort();
        }
        match end {
            End::Exited => {
                drop(proc);
                status(&sink, LspStatus::Stopped);
                return;
            }
            End::Closed => {
                stop_process(proc, config.stop_grace).await;
                return;
            }
            End::Stopped(done) => {
                stop_process(proc, config.stop_grace).await;
                status(&sink, LspStatus::Stopped);
                if let Some(d) = done {
                    let _ = d.send(());
                }
                return;
            }
            End::Crashed(how) => {
                drop(proc);
                let now = Instant::now();
                crashes.push_back(now);
                while crashes
                    .front()
                    .is_some_and(|t| now.duration_since(*t) > config.crash_window)
                {
                    crashes.pop_front();
                }
                let last = last_err
                    .lock()
                    .unwrap_or_else(|e| e.into_inner())
                    .back()
                    .cloned();
                if crashes.len() >= config.max_crashes {
                    let reason = match last {
                        Some(l) => format!("{} keeps crashing ({how}): {l}", spec.name),
                        None => format!("{} keeps crashing ({how})", spec.name),
                    };
                    status(&sink, LspStatus::Unavailable { reason });
                    return;
                }
                let attempt = u32::try_from(crashes.len()).unwrap_or(u32::MAX);
                let delay = config
                    .backoff_base
                    .saturating_mul(1 << (attempt - 1).min(16))
                    .min(config.backoff_max);
                status(
                    &sink,
                    LspStatus::Restarting {
                        attempt,
                        delay_ms: u64::try_from(delay.as_millis()).unwrap_or(u64::MAX),
                    },
                );
                // Wait out the backoff; messages meanwhile are dropped (the UI reconnects).
                let resume_at = Instant::now() + delay;
                loop {
                    tokio::select! {
                        _ = sleep_until(resume_at) => break,
                        cmd = rx.recv() => match cmd {
                            None => return,
                            Some(Cmd::Stop(done)) => {
                                status(&sink, LspStatus::Stopped);
                                let _ = done.send(());
                                return;
                            }
                            Some(Cmd::SetSink(s)) => sink = s,
                            Some(Cmd::Send(_)) => {}
                        }
                    }
                }
            }
        }
    }
}

fn exit_text(code: Option<i32>) -> String {
    match code {
        Some(c) => format!("exit code {c}"),
        None => "terminated".into(),
    }
}
