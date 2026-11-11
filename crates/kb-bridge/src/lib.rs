//! Bridges language servers (LSP) and, later, debug adapters (DAP) to the UI (ADR 0014).
//!
//! Both protocols use `Content-Length` framed JSON-RPC over stdio ([`framing`]).
//! Messages are relayed to the UI over Tauri channels (no localhost ports).
//!
//! Language servers run **outside** the judge sandbox, so they must never execute user
//! code:
//! - [`spec`] holds the settings that guarantee it, and the clean environment;
//! - [`project`] holds the per-problem virtual project they work in;
//! - [`policy`] filters every message both ways (pinned root, allow-listed methods,
//!   documents only inside the project).
//!
//! [`session`] runs one server: lazy start, idle stop, restart with backoff.
//! [`manager`] keeps one per language.

pub mod framing;
pub mod manager;
pub mod policy;
pub mod process;
pub mod project;
pub mod session;
pub mod spec;

pub use manager::{BridgeError, LspConnection, LspManager, SpecFn};
pub use session::{LspEvent, LspStatus, SessionConfig, Sink};
pub use spec::{ServerSpec, Toolchains};
