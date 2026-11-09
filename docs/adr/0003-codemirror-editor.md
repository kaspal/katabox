# 3. CodeMirror 6 as the editor

Date: 2026-11-08 · Status: accepted

## Decision
CodeMirror 6 with @codemirror/lsp-client, relaying LSP over Tauri channels. Language servers: basedpyright, typescript-language-server, phpactor, rust-analyzer.

## Consequences
Smaller and faster on WebKitGTK than Monaco; transport-agnostic LSP client. We build our own debugger UI on DAP types.
