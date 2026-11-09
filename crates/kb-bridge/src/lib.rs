//! Bridges language servers (LSP) and debug adapters (DAP) to the UI.
//!
//! Both protocols use `Content-Length` framed JSON-RPC over stdio, so one
//! framing module serves both; messages are relayed to the UI over Tauri
//! channels (no localhost ports).
