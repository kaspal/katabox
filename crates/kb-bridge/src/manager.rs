//! All language servers of the app: one session per language, started lazily on the
//! first connect, bound to the problem being edited.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use kb_core::Language;
use serde::Serialize;
use tokio::sync::Mutex;

use crate::project::{self, Project};
use crate::session::{LspEvent, LspStatus, Session, SessionConfig, Sink};
use crate::spec::{self, ServerSpec, Toolchains};

#[derive(Debug, thiserror::Error)]
pub enum BridgeError {
    #[error("invalid problem id {0:?}")]
    InvalidProblem(String),
    #[error("{0}")]
    Unavailable(String),
    #[error("no {0} language server is connected")]
    NotConnected(Language),
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

/// What the editor needs to talk about its document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "specta", derive(specta::Type))]
pub struct LspConnection {
    pub root_uri: String,
    pub file_uri: String,
    pub language_id: String,
}

/// Builds the server for a language (replaceable in tests).
pub type SpecFn =
    Arc<dyn Fn(Language, &Project, &Path) -> Result<ServerSpec, String> + Send + Sync>;

pub struct LspManager {
    tc: Toolchains,
    projects_root: PathBuf,
    state_root: PathBuf,
    config: SessionConfig,
    spec: Option<SpecFn>,
    sessions: Mutex<HashMap<Language, Session>>,
}

/// JavaScript files are served by the TypeScript server.
fn server_language(l: Language) -> Language {
    if l == Language::JavaScript {
        Language::TypeScript
    } else {
        l
    }
}

impl LspManager {
    /// `root`: the folder with `toolchains/` and `tools/`; projects and per-server state
    /// go under `data_root` (`lsp/` and `lsp-state/`).
    pub fn new(root: impl Into<PathBuf>, data_root: &Path) -> Self {
        LspManager {
            tc: Toolchains::new(root),
            projects_root: data_root.join("lsp"),
            state_root: data_root.join("lsp-state"),
            config: SessionConfig::default(),
            spec: None,
            sessions: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_config(mut self, config: SessionConfig) -> Self {
        self.config = config;
        self
    }

    pub fn with_spec(mut self, spec: SpecFn) -> Self {
        self.spec = Some(spec);
        self
    }

    pub fn toolchains(&self) -> &Toolchains {
        &self.tc
    }

    /// Why `language` has no server here, if it hasn't.
    pub fn unavailable_reason(&self, language: Language) -> Option<String> {
        if self.spec.is_some() {
            return None;
        }
        spec::unavailable_reason(server_language(language), &self.tc)
    }

    /// Start (or rebind) the server for `language` on `problem`. Events go to `sink`.
    /// The solution file is written with `text` when a new session starts.
    pub async fn connect(
        &self,
        problem: &str,
        language: Language,
        text: &str,
        sink: Sink,
    ) -> Result<LspConnection, BridgeError> {
        if !project::valid_problem_id(problem) {
            return Err(BridgeError::InvalidProblem(problem.to_string()));
        }
        let key = server_language(language);
        let mut sessions = self.sessions.lock().await;
        if let Some(s) = sessions.get(&key) {
            if s.problem == problem && s.is_alive() {
                s.set_sink(sink);
                return Ok(connection(&s.project));
            }
        }
        if let Some(old) = sessions.remove(&key) {
            old.stop().await;
        }
        let project = project::prepare(&self.projects_root, problem, language, text, &self.tc)?;
        let state = self.state_root.join(key.id());
        let made = match &self.spec {
            Some(f) => f(key, &project, &state),
            None => spec::server_for(key, &self.tc, &project, &state),
        };
        let spec = match made {
            Ok(s) => s,
            Err(reason) => {
                sink(LspEvent::Status {
                    status: LspStatus::Unavailable {
                        reason: reason.clone(),
                    },
                });
                return Err(BridgeError::Unavailable(reason));
            }
        };
        let conn = connection(&project);
        let session = Session::start(
            problem.to_string(),
            project,
            spec,
            sink,
            self.config.clone(),
        );
        sessions.insert(key, session);
        Ok(conn)
    }

    /// Relay a JSON-RPC message from the UI.
    pub async fn send(&self, language: Language, message: String) -> Result<(), BridgeError> {
        let key = server_language(language);
        let sessions = self.sessions.lock().await;
        match sessions.get(&key) {
            Some(s) if s.send(message) => Ok(()),
            _ => Err(BridgeError::NotConnected(key)),
        }
    }

    pub async fn disconnect(&self, language: Language) {
        let removed = self
            .sessions
            .lock()
            .await
            .remove(&server_language(language));
        if let Some(s) = removed {
            s.stop().await;
        }
    }

    /// Stop every server (app exit).
    pub async fn shutdown(&self) {
        let all: Vec<Session> = self.sessions.lock().await.drain().map(|(_, s)| s).collect();
        for s in all {
            s.stop().await;
        }
    }
}

fn connection(p: &Project) -> LspConnection {
    LspConnection {
        root_uri: p.root_uri.clone(),
        file_uri: p.file_uri.clone(),
        language_id: p.language_id.to_string(),
    }
}
